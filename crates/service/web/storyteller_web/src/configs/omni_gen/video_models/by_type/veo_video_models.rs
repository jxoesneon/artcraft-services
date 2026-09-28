use artcraft_api_defs::omni_gen::models::omni_gen_video_models::OmniGenVideoModelDetails;
use enums::common::generation::common_aspect_ratio::CommonAspectRatio;
use enums::common::generation::common_resolution::CommonResolution;
use enums::common::generation::common_video_model::CommonVideoModel;
use enums::common::generation::model_creator::ModelCreator;

/// Veo (Google) video models.
pub fn veo_video_models() -> Vec<OmniGenVideoModelDetails> {
  let mut models = Vec::new();

  models.push(OmniGenVideoModelDetails {
    model: CommonVideoModel::Veo3p1,
    model_creator: Some(ModelCreator::Google),
    full_name: Some("Veo 3.1".to_string()),
    text_prompt_supported: Some(true),
    starting_keyframe_supported: Some(true),
    ending_keyframe_supported: Some(true),
    image_references_supported: Some(true),
    image_references_max: Some(3),
    video_references_supported: Some(true),
    video_references_max: Some(1),
    show_generate_with_sound_toggle: Some(true),
    aspect_ratio_options: Some(vec![
      CommonAspectRatio::Auto,
      CommonAspectRatio::WideSixteenByNine,
      CommonAspectRatio::TallNineBySixteen,
    ]),
    aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
    resolution_options: Some(vec![
      CommonResolution::SevenTwentyP,
      CommonResolution::TenEightyP,
      CommonResolution::FourK,
    ]),
    resolution_default: Some(CommonResolution::TenEightyP),
    // fal accepts only 4s/6s/8s (reference-to-video: 8s only; see the router's veo_3p1_common).
    duration_seconds_options: Some(vec![4, 6, 8]),
    duration_seconds_default: Some(8),
    ..Default::default()
  });

  models.push(OmniGenVideoModelDetails {
    model: CommonVideoModel::Veo3p1Fast,
    model_creator: Some(ModelCreator::Google),
    full_name: Some("Veo 3.1 Fast".to_string()),
    text_prompt_supported: Some(true),
    starting_keyframe_supported: Some(true),
    ending_keyframe_supported: Some(true),
    image_references_supported: Some(true),
    image_references_max: Some(3),
    video_references_supported: Some(true),
    video_references_max: Some(1),
    show_generate_with_sound_toggle: Some(true),
    aspect_ratio_options: Some(vec![
      CommonAspectRatio::Auto,
      CommonAspectRatio::WideSixteenByNine,
      CommonAspectRatio::TallNineBySixteen,
    ]),
    aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
    resolution_options: Some(vec![
      CommonResolution::SevenTwentyP,
      CommonResolution::TenEightyP,
      CommonResolution::FourK,
    ]),
    resolution_default: Some(CommonResolution::TenEightyP),
    // fal accepts only 4s/6s/8s (reference-to-video: 8s only; see the router's veo_3p1_common).
    duration_seconds_options: Some(vec![4, 6, 8]),
    duration_seconds_default: Some(8),
    ..Default::default()
  });

  models.push(OmniGenVideoModelDetails {
    model: CommonVideoModel::Veo3p1Lite,
    model_creator: Some(ModelCreator::Google),
    full_name: Some("Veo 3.1 Lite".to_string()),
    text_prompt_supported: Some(true),
    starting_keyframe_supported: Some(true),
    ending_keyframe_supported: Some(true),
    show_generate_with_sound_toggle: Some(true),
    aspect_ratio_options: Some(vec![
      CommonAspectRatio::Auto,
      CommonAspectRatio::WideSixteenByNine,
      CommonAspectRatio::TallNineBySixteen,
    ]),
    aspect_ratio_default: Some(CommonAspectRatio::WideSixteenByNine),
    resolution_options: Some(vec![
      CommonResolution::SevenTwentyP,
      CommonResolution::TenEightyP,
    ]),
    resolution_default: Some(CommonResolution::SevenTwentyP),
    // fal accepts only 4s/6s/8s (reference-to-video: 8s only; see the router's veo_3p1_common).
    duration_seconds_options: Some(vec![4, 6, 8]),
    duration_seconds_default: Some(8),
    ..Default::default()
  });

  models
}
