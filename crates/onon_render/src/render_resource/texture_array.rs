use crate::render_resource::Texture;
use std::collections::VecDeque;

pub struct TextureArray {
  pub texture: Texture,
  pub bind_group: wgpu::BindGroup,

  layer_capacity: u32,
  free_slots: VecDeque<u32>,
}

impl TextureArray {
  pub fn srgba8_texture(
    device: &wgpu::Device,
    size: wgpu::Extent3d,
    sampler: std::sync::Arc<wgpu::Sampler>,
    bind_group_layout: &wgpu::BindGroupLayout
  ) -> Self {
    let texture = Texture::create_array(&device, sampler, size, wgpu::TextureFormat::Rgba8UnormSrgb);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&texture.view()),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&texture.sampler()),
        },
      ],
      label: Some("diffuse_bind_group"),
    });

    Self {
      texture,
      bind_group,
      layer_capacity: size.depth_or_array_layers,
      free_slots: (0..size.depth_or_array_layers).collect(),
    }
  }

  pub fn upload_texture(
    &mut self,
    queue: &wgpu::Queue,
    data: &[u8],
    width: u32,
    height: u32,
  ) -> u32 {
    let slot = self.free_slots.pop_front().expect("Texture Array is full!");

    queue.write_texture(
      wgpu::TexelCopyTextureInfo {
        texture: self.texture.texture(),
        mip_level: 0,
        origin: wgpu::Origin3d {
          x: 0,
          y: 0,
          z: slot,
        },
        aspect: wgpu::TextureAspect::All,
      },
      data,
      wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(width * 4),
        rows_per_image: Some(height),
      },
      wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
      },
    );

    slot
  }
}
