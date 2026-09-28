//! Veo 3.1 duration rules, shared by the Artcraft provider (which prices a
//! request) and the fal provider (which executes it) so both always agree on
//! the duration that will actually be generated and billed.
//!
//! fal's Veo 3.1 endpoints don't all accept the same durations. From fal's
//! OpenAPI schemas (checked 2026-09-28):
//!
//! | Modality                  | Veo 3.1 / 3.1 Fast | Veo 3.1 Lite |
//! |---------------------------|--------------------|--------------|
//! | text-to-video             | 4s, 6s, 8s         | 4s, 6s, 8s   |
//! | image-to-video            | 4s, 6s, 8s         | 4s, 6s, 8s   |
//! | first-last-frame-to-video | 4s, 6s, 8s         | 8s only      |
//! | reference-to-video        | 8s only            | (none)       |
//! | extend-video              | 7s only            | (none)       |
//!
//! Sending another value fails at fal, eg. `Input should be '8s'`.

use crate::api::image_list_ref::ImageListRef;
use crate::api::video_list_ref::VideoListRef;
use crate::client::request_mismatch_mitigation_strategy::RequestMismatchMitigationStrategy;
use crate::errors::artcraft_router_error::ArtcraftRouterError;
use crate::errors::client_error::ClientError;
use crate::generate::generate_video::generate_video_request_builder::GenerateVideoRequestBuilder;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Veo3p1Variant {
  Standard,
  Fast,
  Lite,
}

/// The fal endpoint a request goes to. Mirrors the fal builders' dispatch
/// order: reference videos, then reference images, then frames.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Veo3p1Modality {
  TextToVideo,
  ImageToVideo,
  FirstLastFrameToVideo,
  ReferenceToVideo,
  ExtendVideo,
}

impl Veo3p1Modality {
  pub(crate) fn of(builder: &GenerateVideoRequestBuilder) -> Self {
    if has_videos(&builder.reference_videos) {
      Veo3p1Modality::ExtendVideo
    } else if has_images(&builder.reference_images) {
      Veo3p1Modality::ReferenceToVideo
    } else {
      match (builder.start_frame.is_some(), builder.end_frame.is_some()) {
        (true, true) => Veo3p1Modality::FirstLastFrameToVideo,
        (true, false) => Veo3p1Modality::ImageToVideo,
        // (false, true) is rejected by the fal builders.
        (false, _) => Veo3p1Modality::TextToVideo,
      }
    }
  }
}

/// The durations (seconds, ascending) fal accepts for this variant and modality.
pub(crate) fn veo_3p1_allowed_durations(variant: Veo3p1Variant, modality: Veo3p1Modality) -> &'static [u16] {
  match (variant, modality) {
    (_, Veo3p1Modality::ExtendVideo) => &[7],
    (_, Veo3p1Modality::ReferenceToVideo) => &[8],
    (Veo3p1Variant::Lite, Veo3p1Modality::FirstLastFrameToVideo) => &[8],
    _ => &[4, 6, 8],
  }
}

/// Plan the duration to send for a request.
///
/// A supported duration passes through. Otherwise `ErrorOut` fails, and the
/// other strategies pick the nearest supported duration above
/// (`PayMoreUpgrade`) or below (`PayLessDowngrade`), clamped to the range.
/// With no requested duration, returns fal's default for the endpoint (the
/// longest allowed: 8s, or 7s for extend-video), explicitly, so the price is
/// computed from the duration that will actually be generated.
pub(crate) fn plan_veo_3p1_duration(
  variant: Veo3p1Variant,
  modality: Veo3p1Modality,
  duration_seconds: Option<u16>,
  strategy: RequestMismatchMitigationStrategy,
) -> Result<Option<u16>, ArtcraftRouterError> {
  let allowed = veo_3p1_allowed_durations(variant, modality);
  let shortest = allowed[0];
  let longest = allowed[allowed.len() - 1];
  let requested = match duration_seconds {
    None => return Ok(Some(longest)),
    Some(requested) => requested,
  };
  if allowed.contains(&requested) {
    return Ok(Some(requested));
  }
  match strategy {
    RequestMismatchMitigationStrategy::ErrorOut => Err(ArtcraftRouterError::Client(ClientError::ModelDoesNotSupportOption {
      field: "duration_seconds",
      value: format!("{} (supported for this input: {:?})", requested, allowed),
    })),
    RequestMismatchMitigationStrategy::PayMoreUpgrade => Ok(Some(
      allowed.iter().copied().find(|&allowed| allowed >= requested).unwrap_or(longest))),
    RequestMismatchMitigationStrategy::PayLessDowngrade => Ok(Some(
      allowed.iter().copied().rev().find(|&allowed| allowed <= requested).unwrap_or(shortest))),
  }
}

fn has_images(refs: &Option<ImageListRef>) -> bool {
  match refs {
    None => false,
    Some(ImageListRef::Urls(urls)) => !urls.is_empty(),
    Some(ImageListRef::MediaFileTokens(tokens)) => !tokens.is_empty(),
  }
}

fn has_videos(refs: &Option<VideoListRef>) -> bool {
  match refs {
    None => false,
    Some(VideoListRef::Urls(urls)) => !urls.is_empty(),
    Some(VideoListRef::MediaFileTokens(tokens)) => !tokens.is_empty(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use RequestMismatchMitigationStrategy::{ErrorOut, PayLessDowngrade, PayMoreUpgrade};
  use Veo3p1Modality::*;
  use Veo3p1Variant::*;

  mod allowed_durations {
    use super::*;

    #[test]
    fn standard_and_fast() {
      for variant in [Standard, Fast] {
        assert_eq!(veo_3p1_allowed_durations(variant, TextToVideo), &[4, 6, 8]);
        assert_eq!(veo_3p1_allowed_durations(variant, ImageToVideo), &[4, 6, 8]);
        assert_eq!(veo_3p1_allowed_durations(variant, FirstLastFrameToVideo), &[4, 6, 8]);
        assert_eq!(veo_3p1_allowed_durations(variant, ReferenceToVideo), &[8]);
        assert_eq!(veo_3p1_allowed_durations(variant, ExtendVideo), &[7]);
      }
    }

    #[test]
    fn lite() {
      assert_eq!(veo_3p1_allowed_durations(Lite, TextToVideo), &[4, 6, 8]);
      assert_eq!(veo_3p1_allowed_durations(Lite, ImageToVideo), &[4, 6, 8]);
      assert_eq!(veo_3p1_allowed_durations(Lite, FirstLastFrameToVideo), &[8]);
    }
  }

  mod planning {
    use super::*;

    #[test]
    fn supported_durations_pass_through() {
      for seconds in [4, 6, 8] {
        assert_eq!(plan_veo_3p1_duration(Fast, TextToVideo, Some(seconds), ErrorOut).unwrap(), Some(seconds));
      }
    }

    #[test]
    fn reference_to_video_is_always_eight_seconds() {
      // The production bug: 4s/6s sent to reference-to-video ("Input should be '8s'").
      for seconds in [4, 6] {
        assert_eq!(plan_veo_3p1_duration(Fast, ReferenceToVideo, Some(seconds), PayMoreUpgrade).unwrap(), Some(8));
        assert_eq!(plan_veo_3p1_duration(Fast, ReferenceToVideo, Some(seconds), PayLessDowngrade).unwrap(), Some(8));
        assert!(plan_veo_3p1_duration(Fast, ReferenceToVideo, Some(seconds), ErrorOut).is_err());
      }
      assert_eq!(plan_veo_3p1_duration(Fast, ReferenceToVideo, None, ErrorOut).unwrap(), Some(8));
    }

    #[test]
    fn extend_video_is_always_seven_seconds() {
      assert_eq!(plan_veo_3p1_duration(Standard, ExtendVideo, Some(8), PayMoreUpgrade).unwrap(), Some(7));
      assert_eq!(plan_veo_3p1_duration(Standard, ExtendVideo, Some(4), PayMoreUpgrade).unwrap(), Some(7));
      assert_eq!(plan_veo_3p1_duration(Standard, ExtendVideo, None, ErrorOut).unwrap(), Some(7));
    }

    #[test]
    fn lite_first_last_frame_is_always_eight_seconds() {
      assert_eq!(plan_veo_3p1_duration(Lite, FirstLastFrameToVideo, Some(4), PayMoreUpgrade).unwrap(), Some(8));
    }

    #[test]
    fn unsupported_durations_round_to_the_nearest_supported_one() {
      assert_eq!(plan_veo_3p1_duration(Fast, TextToVideo, Some(5), PayMoreUpgrade).unwrap(), Some(6));
      assert_eq!(plan_veo_3p1_duration(Fast, TextToVideo, Some(7), PayMoreUpgrade).unwrap(), Some(8));
      assert_eq!(plan_veo_3p1_duration(Fast, TextToVideo, Some(12), PayMoreUpgrade).unwrap(), Some(8));
      assert_eq!(plan_veo_3p1_duration(Fast, TextToVideo, Some(5), PayLessDowngrade).unwrap(), Some(4));
      assert_eq!(plan_veo_3p1_duration(Fast, TextToVideo, Some(2), PayLessDowngrade).unwrap(), Some(4));
      assert!(plan_veo_3p1_duration(Fast, TextToVideo, Some(5), ErrorOut).is_err());
    }

    #[test]
    fn missing_duration_is_fals_default() {
      // fal defaults every Veo 3.1 endpoint to 8s, except extend-video (7s).
      for modality in [TextToVideo, ImageToVideo, FirstLastFrameToVideo, ReferenceToVideo] {
        assert_eq!(plan_veo_3p1_duration(Standard, modality, None, ErrorOut).unwrap(), Some(8), "{:?}", modality);
      }
      assert_eq!(plan_veo_3p1_duration(Standard, ExtendVideo, None, ErrorOut).unwrap(), Some(7));
    }
  }
}
