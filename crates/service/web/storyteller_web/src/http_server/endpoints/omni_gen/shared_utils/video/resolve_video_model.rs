use enums::common::generation::common_video_model::CommonVideoModel;

use crate::http_server::common_responses::common_web_error::CommonWebError;

/// Whether a video model can still be generated, and how.
///
/// The `CommonVideoModel` variants below stay in the enum forever: they're
/// stored in historical database records. They're only removed from the model
/// list (`configs::omni_gen::video_models`) and handled here on requests.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VideoModelStatus {
  Available,
  /// Fulfilled (and priced) as a different model.
  ReplacedBy(CommonVideoModel),
  /// No longer offered; requests get a 400.
  Retired { display_name: &'static str },
}

/// Resolve the requested model before any pricing, validation, or billing:
/// replaced models are rewritten to their replacement, and retired models are
/// rejected with a 400. Call this first in every video generate/cost handler.
pub fn resolve_video_model(
  maybe_model: Option<CommonVideoModel>,
) -> Result<Option<CommonVideoModel>, CommonWebError> {
  let Some(model) = maybe_model else {
    return Ok(None);
  };
  match video_model_status(model) {
    VideoModelStatus::Available => Ok(Some(model)),
    VideoModelStatus::ReplacedBy(replacement) => Ok(Some(replacement)),
    VideoModelStatus::Retired { display_name } => Err(CommonWebError::BadInputWithSimpleMessage(
      format!("{display_name} is no longer available. Please choose a different video model."))),
  }
}

/// Retired models were pulled by their upstream provider; fal marks their
/// endpoints `deprecated` (checked 2026-09-28 against fal's model API).
/// Where a newer model in the same family is available, requests are upgraded
/// to it instead of failing.
pub fn video_model_status(model: CommonVideoModel) -> VideoModelStatus {
  match model {
    // Sora 2 is no longer available from the API (2026-09-28).
    CommonVideoModel::Sora2 => VideoModelStatus::Retired { display_name: "Sora 2" },
    CommonVideoModel::Sora2Pro => VideoModelStatus::Retired { display_name: "Sora 2 Pro" },

    // Veo 2, Veo 3 and Veo 3 Fast endpoints deprecated (2026-09-28).
    CommonVideoModel::Veo2 => VideoModelStatus::ReplacedBy(CommonVideoModel::Veo3p1),
    CommonVideoModel::Veo3 => VideoModelStatus::ReplacedBy(CommonVideoModel::Veo3p1),
    CommonVideoModel::Veo3Fast => VideoModelStatus::ReplacedBy(CommonVideoModel::Veo3p1Fast),

    // Kling 1.6 Pro and 2.1 Pro/Master endpoints deprecated (2026-09-28).
    CommonVideoModel::Kling16Pro => VideoModelStatus::ReplacedBy(CommonVideoModel::Kling2p5TurboPro),
    CommonVideoModel::Kling21Pro => VideoModelStatus::ReplacedBy(CommonVideoModel::Kling2p5TurboPro),
    CommonVideoModel::Kling21Master => VideoModelStatus::ReplacedBy(CommonVideoModel::Kling2p6Pro),

    // Seedance 1.0 Lite endpoint deprecated (2026-09-28).
    CommonVideoModel::Seedance10Lite => VideoModelStatus::ReplacedBy(CommonVideoModel::Seedance1p5Pro),

    // Seedance 2.5 Preview was superseded by Seedance 2.5 (2026-09-28). It's
    // fulfilled and priced as Seedance 2.5, which is cheaper.
    CommonVideoModel::Seedance2p5Preview => VideoModelStatus::ReplacedBy(CommonVideoModel::Seedance2p5),

    _ => VideoModelStatus::Available,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod retired {
    use super::*;

    #[test]
    fn sora_models_are_rejected_with_a_400() {
      for (model, name) in [(CommonVideoModel::Sora2, "Sora 2"), (CommonVideoModel::Sora2Pro, "Sora 2 Pro")] {
        match resolve_video_model(Some(model)) {
          Err(CommonWebError::BadInputWithSimpleMessage(message)) => {
            assert_eq!(message, format!("{name} is no longer available. Please choose a different video model."));
          }
          other => panic!("expected a 400 for {:?}, got {:?}", model, other),
        }
      }
    }
  }

  mod replaced {
    use super::*;

    #[test]
    fn replaced_models_resolve_to_their_replacement() {
      for (model, replacement) in [
        (CommonVideoModel::Veo2, CommonVideoModel::Veo3p1),
        (CommonVideoModel::Veo3, CommonVideoModel::Veo3p1),
        (CommonVideoModel::Veo3Fast, CommonVideoModel::Veo3p1Fast),
        (CommonVideoModel::Kling16Pro, CommonVideoModel::Kling2p5TurboPro),
        (CommonVideoModel::Kling21Pro, CommonVideoModel::Kling2p5TurboPro),
        (CommonVideoModel::Kling21Master, CommonVideoModel::Kling2p6Pro),
        (CommonVideoModel::Seedance10Lite, CommonVideoModel::Seedance1p5Pro),
        (CommonVideoModel::Seedance2p5Preview, CommonVideoModel::Seedance2p5),
      ] {
        assert_eq!(resolve_video_model(Some(model)).unwrap(), Some(replacement), "{model:?}");
      }
    }

    #[test]
    fn replacements_are_themselves_available() {
      for model in [
        CommonVideoModel::Veo2, CommonVideoModel::Veo3, CommonVideoModel::Veo3Fast,
        CommonVideoModel::Kling16Pro, CommonVideoModel::Kling21Pro, CommonVideoModel::Kling21Master,
        CommonVideoModel::Seedance10Lite, CommonVideoModel::Seedance2p5Preview,
      ] {
        let VideoModelStatus::ReplacedBy(replacement) = video_model_status(model) else {
          panic!("{:?} should be replaced", model);
        };
        assert_eq!(video_model_status(replacement), VideoModelStatus::Available, "{model:?} -> {replacement:?}");
      }
    }
  }

  #[test]
  fn available_and_missing_models_pass_through() {
    assert_eq!(resolve_video_model(Some(CommonVideoModel::Veo3p1Fast)).unwrap(), Some(CommonVideoModel::Veo3p1Fast));
    assert_eq!(resolve_video_model(Some(CommonVideoModel::Seedance2p5)).unwrap(), Some(CommonVideoModel::Seedance2p5));
    assert_eq!(resolve_video_model(None).unwrap(), None);
  }
}
