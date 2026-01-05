use crate::{
  mesh::Mesh2D
};

pub struct RenderObject {
  pub mesh: Mesh2D,
  pub pipeline_id: u32,
  pub texture_array_id: u32,
  pub texture_id: u32
}

impl RenderObject {
  pub fn new(mesh: Mesh2D, pipeline_id: u32, texture_array_id: u32, texture_id: u32) -> Self {
    Self{mesh, pipeline_id, texture_array_id, texture_id}
  }
}