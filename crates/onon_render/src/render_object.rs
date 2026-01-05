use crate::{mesh::Mesh2D, render_resource::texture_array::TextureArrayInfo};

pub struct RenderObject {
  pub mesh: Mesh2D,
  pub pipeline_id: u32,
  pub texture_array_info: Option<TextureArrayInfo>,
  pub texture_slot: Option<u32>,
  pub texture_path: Option<std::path::PathBuf>,
}

impl RenderObject {
  pub fn new(
    mesh: Mesh2D,
    pipeline_id: u32,
    texture_array_info: Option<TextureArrayInfo>,
    texture_path: Option<std::path::PathBuf>,
    texture_slot: Option<u32>
  ) -> Self {
    Self {
      mesh,
      pipeline_id,
      texture_array_info,
      texture_path,
      texture_slot,
    }
  }
}
