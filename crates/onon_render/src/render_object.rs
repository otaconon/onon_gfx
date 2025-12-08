use std::sync::Arc;

use crate::{
  mesh::Mesh2D, shader_pass::ShaderPass
};

pub struct RenderObject {
  pub mesh: Mesh2D,
  pub shader_pass: Arc<ShaderPass>
}

impl RenderObject {
  pub fn new(mesh: Mesh2D, shader_pass: Arc<ShaderPass>) -> Self {
    Self{mesh, shader_pass}
  }
}