use std::sync::Arc;
use wgpu::{CommandEncoder, include_wgsl};
use winit::{dpi::PhysicalSize, window::Window};
use crate::{queries, render_object::RenderObject, render_resource::{FrameContext, render_pipeline}};

struct RenderContext<'a> {
  encoder: CommandEncoder,
  render_pass: Option<wgpu::RenderPass<'a>>
}

pub struct Renderer {
  window: Arc<Window>,
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  resize_requested: bool,
  pub size: PhysicalSize<u32>, //Remove from here
}

impl Renderer {
  pub async fn new(window: Arc<Window>) -> Self {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });
    let surface = instance.create_surface(window.clone()).unwrap();
    let adapter = queries::query_adapter(&instance, &surface).await;
    let (device, queue) = queries::query_device(&adapter).await;

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.width.max(1);
    let config = get_surface_config(surface.get_capabilities(&adapter), size);
    surface.configure(&device, &config);

    let shader = device.create_shader_module(include_wgsl!("../../../shaders/triangle.wgsl"));
    let layout = render_pipeline::create_layout(&device);
    let render_pipeline = render_pipeline::create_pipeline(&device, &layout, &shader, &config);

    Self {window, surface, device, queue, config, resize_requested: false, size}
  }

  pub fn begin_rendering(&mut self) -> Result<Option<FrameContext>, wgpu::SurfaceError> {
    if self.size.width == 0 || self.size.height == 0 {
      return Ok(None);
    } else if self.resize_requested {
      self.resize();
    } 

    let output = self.surface.get_current_texture()?;


    let encoder = self
      .device 
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });
    
    Ok(Some(FrameContext::new(encoder, output, self.queue.clone())))
  }

  pub fn render_objects(&self, render_pass: &mut wgpu::RenderPass, objects: &Vec<RenderObject>) {
    for object in objects {
      render_pass.set_pipeline(&object.shader_pass.render_pipeline);
      render_pass.draw(0..3, 0..1);
    }
  }

  pub fn finish_rendering(&self, frame_ctx: FrameContext) {
    self.queue.submit(Some(frame_ctx.encoder.finish()));
    frame_ctx.output.present();
  }

  pub fn request_resize(&mut self) {
    self.resize_requested = true;
  }

  fn resize(&mut self) {
    self.config.width = self.size.width;
    self.config.height = self.size.height;
    self.surface.configure(&self.device, &self.config);
    self.resize_requested = false;
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
