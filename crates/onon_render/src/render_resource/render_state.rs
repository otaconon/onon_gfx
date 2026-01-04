use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use crate::queries;

pub struct RenderState<'a> {
  pub surface: wgpu::Surface<'a>,
  device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
  pub new_size: Option<PhysicalSize<u32>>
}

impl<'a> RenderState<'a> {
  pub async fn new(window: Arc<Window>) -> Self {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.width.max(1);

    let surface = instance.create_surface(window.clone()).unwrap();
    let adapter = queries::query_adapter(&instance, &surface).await;
    let (device, queue) = queries::query_device(&adapter).await;

    let config = get_surface_config(surface.get_capabilities(&adapter), size);
    surface.configure(&device, &config);

    Self {surface, device, queue, config, new_size: None}
  }

  pub fn resize(&mut self) {
    match self.new_size {
      Some(size) => {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
      }
      None => {}
    }
  }

  pub fn get_size(&self) -> PhysicalSize<u32> {
    PhysicalSize{width: self.config.width, height: self.config.height}
  }

  pub fn device(&self) -> &wgpu::Device {
    &self.device
  }
}

fn get_surface_config(
  capabilities: wgpu::SurfaceCapabilities,
  size: PhysicalSize<u32>,
) -> wgpu::SurfaceConfiguration {
  wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: capabilities.formats[0],
    width: size.width,
    height: size.height,
    present_mode: wgpu::PresentMode::Fifo,
    alpha_mode: capabilities.alpha_modes[0],
    view_formats: vec![],
    desired_maximum_frame_latency: 2,
  }
}