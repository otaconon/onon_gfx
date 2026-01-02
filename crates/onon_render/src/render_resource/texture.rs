use std::{path::Path};
use anyhow::{Context, Result};

pub struct Texture {
  device: wgpu::Device
}

impl Texture {
  pub fn new(device: wgpu::Device) -> Self {
    Self{device}
  }

  pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
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

    Ok(())
  }
}