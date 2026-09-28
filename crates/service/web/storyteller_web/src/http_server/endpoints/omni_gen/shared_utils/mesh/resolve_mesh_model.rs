use enums::common::generation::common_mesh_model::CommonMeshModel;

use crate::http_server::common_responses::common_web_error::CommonWebError;

/// Whether a mesh model can still be generated, and how.
///
/// The `CommonMeshModel` variants below stay in the enum forever: they're
/// stored in historical database records. They're only removed from the model
/// list (`configs::omni_gen::mesh_models`) and handled here on requests.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MeshModelStatus {
  Available,
  /// Fulfilled (and priced) as a different model.
  ReplacedBy(CommonMeshModel),
}

/// Resolve the requested model before any pricing, validation, or billing:
/// replaced models are rewritten to their replacement. Call this first in
/// every mesh generate/cost handler.
///
/// (No mesh model is retired outright yet. When one is, add a `Retired`
/// status that returns a 400, like `resolve_video_model`.)
pub fn resolve_mesh_model(
  maybe_model: Option<CommonMeshModel>,
) -> Result<Option<CommonMeshModel>, CommonWebError> {
  Ok(maybe_model.map(|model| match mesh_model_status(model) {
    MeshModelStatus::Available => model,
    MeshModelStatus::ReplacedBy(replacement) => replacement,
  }))
}

/// Replaced models were pulled by their upstream provider; fal marks their
/// endpoints `deprecated` (checked 2026-09-28 against fal's model API).
pub fn mesh_model_status(model: CommonMeshModel) -> MeshModelStatus {
  match model {
    // Hunyuan 3D 2.1 endpoint deprecated (2026-09-28). Hunyuan 3D 3 also
    // takes a single input image.
    CommonMeshModel::Hunyuan3d2p1 => MeshModelStatus::ReplacedBy(CommonMeshModel::Hunyuan3d3),
    _ => MeshModelStatus::Available,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hunyuan_3d_2p1_is_replaced_by_hunyuan_3d_3() {
    assert_eq!(resolve_mesh_model(Some(CommonMeshModel::Hunyuan3d2p1)).unwrap(), Some(CommonMeshModel::Hunyuan3d3));
    assert_eq!(mesh_model_status(CommonMeshModel::Hunyuan3d3), MeshModelStatus::Available);
  }

  #[test]
  fn available_and_missing_models_pass_through() {
    assert_eq!(resolve_mesh_model(Some(CommonMeshModel::Hunyuan3d2p0)).unwrap(), Some(CommonMeshModel::Hunyuan3d2p0));
    assert_eq!(resolve_mesh_model(None).unwrap(), None);
  }
}
