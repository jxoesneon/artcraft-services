use enums::common::generation::common_video_model::CommonVideoModel as CommonVideoModelEnum;

use crate::errors::artcraft_router_error::ArtcraftRouterError;
use crate::generate::generate_video::generate_video_request_builder::GenerateVideoRequestBuilder;
use crate::generate::generate_video::providers::artcraft::build_common::{
  build_artcraft_omni_video_request, SupportedResolutions, UltraWideSupport,
};
use crate::generate::generate_video::providers::veo_3p1_common::{plan_veo_3p1_duration, Veo3p1Modality, Veo3p1Variant};
use crate::generate::generate_video::providers::artcraft::veo_3p1::request::ArtcraftVeo3p1RequestState;
use crate::generate::generate_video::video_generation_draft_or_request::VideoGenerationDraftOrRequest;
use crate::generate::generate_video::video_generation_request::VideoGenerationRequest;

pub fn build_artcraft_veo_3p1(mut builder: GenerateVideoRequestBuilder) -> Result<VideoGenerationDraftOrRequest, ArtcraftRouterError> {
  // Pre-plan the duration fal will actually generate (it depends on the
  // modality; see `veo_3p1_common`) so the price matches. Also preserve
  // generate_audio (omni-gen helper hardcodes None on output).
  let strategy = builder.request_mismatch_mitigation_strategy;
  let modality = Veo3p1Modality::of(&builder);
  let final_duration = plan_veo_3p1_duration(Veo3p1Variant::Standard, modality, builder.duration_seconds, strategy)?;
  builder.duration_seconds = final_duration;

  let generate_audio = builder.generate_audio;
  let mut request = build_artcraft_omni_video_request(
    builder,
    CommonVideoModelEnum::Veo3p1,
    SupportedResolutions::FullWith4k,
    UltraWideSupport::Unsupported,
  )?;
  request.generate_audio = generate_audio;

  let state = ArtcraftVeo3p1RequestState { request };
  Ok(VideoGenerationDraftOrRequest::Request(VideoGenerationRequest::ArtcraftVeo3p1(state)))
}
