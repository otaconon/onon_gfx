use std::{path::Path};
use anyhow::{Context, Result};

pub struct Texture {
  device: wgpu::Device,
  texture: Option<wgpu::Texture>,
  view: Option<wgpu::TextureView>,
  sampler: Option<wgpu::Sampler>
}

impl Texture {
  pub fn new(device: wgpu::Device) -> Self {
    Self {
      device,
      texture: None,
      view: None,
      sampler: None
    }
  }

  pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P, queue: wgpu::Queue) -> Result<()> {
    let path = path.as_ref();
    let img = image::open(path).with_context(|| format!("Failed to open image at {:?}", path))?;
    let diffuse_rgba = img.to_rgba8();

    use image::GenericImageView;
    let dimensions = img.dimensions();
    let texture_size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1
    };

    let diffuse_texture = self.device.create_texture(
      &wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("diffuse texture"),
        view_formats: &[]
      }
    );

    queue.write_texture(
      wgpu::TexelCopyTextureInfo {
        texture: &diffuse_texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All
      },
      &diffuse_rgba,
      wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1)
      },
      texture_size
    );

    let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let diffuse_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    });

    self.texture = Some(diffuse_texture);
    self.view = Some(diffuse_texture_view);
    self.sampler = Some(diffuse_sampler);

    Ok(())
  }
}