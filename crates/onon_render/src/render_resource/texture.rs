pub struct Texture {
  texture: wgpu::Texture,
  view: wgpu::TextureView,
  sampler: std::sync::Arc<wgpu::Sampler>,
}

impl Texture {
  pub fn create_array(
    device: &wgpu::Device,
    sampler: std::sync::Arc<wgpu::Sampler>,
    size: wgpu::Extent3d,
    format: wgpu::TextureFormat
  ) -> Self {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      label: Some("texture array"),
      view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    Self {
      texture,
      view,
      sampler,
    }
  }

  pub fn from_image(
    device: &wgpu::Device,
    queue: wgpu::Queue,
    img: image::DynamicImage,
    sampler: std::sync::Arc<wgpu::Sampler>
  ) -> Self {
    let diffuse_rgba = img.to_rgba8();

    use image::GenericImageView;
    let dimensions = img.dimensions();
    let size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      label: Some("diffuse texture"),
      view_formats: &[],
    });

    queue.write_texture(
      wgpu::TexelCopyTextureInfo {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      &diffuse_rgba,
      wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1),
      },
      size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    Self {
      texture,
      view,
      sampler
    }
  }

  pub fn texture(&self) -> &wgpu::Texture {
    &self.texture
  }

  pub fn view(&self) -> &wgpu::TextureView {
    &self.view
  }

  pub fn sampler(&self) -> &wgpu::Sampler {
    &self.sampler
  }
}
