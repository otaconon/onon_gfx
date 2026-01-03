use std::collections::VecDeque;

pub struct TextureArray {
  pub texture_array: wgpu::Texture,
  pub view: wgpu::TextureView,
  pub sampler: wgpu::Sampler,
  pub bind_group: wgpu::BindGroup,

  layer_capacity: u32,
  free_slots: VecDeque<u32>
}