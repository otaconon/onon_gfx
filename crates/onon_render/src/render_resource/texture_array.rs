use crate::render_resource::Texture;
use std::collections::{HashMap, VecDeque};
use anyhow::Result;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TextureArrayInfo {
  pub dims: wgpu::Extent3d,
  pub sampler: std::sync::Arc<wgpu::Sampler>,
  pub bind_group_layout: wgpu::BindGroupLayout
}

pub struct TextureArray {
  pub texture: Texture,
  pub bind_group: wgpu::BindGroup,
  info: TextureArrayInfo,

  free_slots: VecDeque<u32>,
  cache: HashMap<std::path::PathBuf, u32>
}

impl TextureArray {
  pub fn new(
    device: &wgpu::Device,
    info: &TextureArrayInfo,
  ) -> Self {
    let texture = Texture::create_array(&device, info.sampler.clone(), info.dims, wgpu::TextureFormat::Rgba8UnormSrgb);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &info.bind_group_layout,
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
      free_slots: (0..info.dims.depth_or_array_layers).collect(),
      info: info.clone(),
      cache: HashMap::new()
    }
  }

  pub fn by_path(&self, path: &std::path::Path) -> Option<&u32> {
    self.cache.get(path)
  }

  pub fn load_from_file<P: AsRef<std::path::Path>>(&mut self, queue: &wgpu::Queue, path: P) -> Result<u32> {
    let path_ref = path.as_ref();

    if let Some(slot) = self.by_path(path_ref) {
      return Ok(*slot)
    }

    let bytes = std::fs::read(path_ref)?;
    let image = image::load_from_memory(&bytes)?;
    let rgba = image.to_rgba8();

    use image::GenericImageView;
    let dimensions = image.dimensions();

    let slot = self.upload_texture(queue, &rgba, dimensions.0, dimensions.1);
    self.cache.insert(path_ref.to_path_buf(), slot);

    Ok(slot)
  }

  /// This function is not recommended to use as it does not cache texture
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
