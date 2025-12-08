use crate::mesh::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Mesh2D {
  pub vertex_buffer: Vec<Vertex>,
  pub index_buffer: Vec<u32>
}

impl Mesh2D {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
    Self {vertex_buffer: vertices, index_buffer: indices} 
  }
}