use crate::{
  mesh::Mesh2D
};

pub struct RenderObject {
  pub mesh: Mesh2D,
  pub pipeline_id: u32
}

impl RenderObject {
  pub fn new(mesh: Mesh2D, pipeline_id: u32) -> Self {
    Self{mesh, pipeline_id}
  }
}