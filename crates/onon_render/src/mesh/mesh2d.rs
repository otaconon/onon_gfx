use crate::mesh::vertex::Vertex;
use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct Mesh2D {
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
}

impl Mesh2D {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>, device: &wgpu::Device) -> Self {
    Self {
      vertex_buffer: device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("Vertex Buffer"),
          contents: bytemuck::cast_slice(&vertices),
          usage: wgpu::BufferUsages::VERTEX,
        }
      ),
      index_buffer: device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("Index Buffer"),
          contents: bytemuck::cast_slice(&indices),
          usage: wgpu::BufferUsages::INDEX,
        }
      ),
      vertices, 
      indices,
    }
  }
}