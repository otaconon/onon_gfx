use std::sync::Arc;
use wgpu::{CommandEncoder, include_wgsl};
use winit::{dpi::PhysicalSize, window::Window};
use crate::{queries, render_object::RenderObject, render_resource::{FrameContext, RenderState, render_pipeline}};

pub struct Renderer<'a> {
  window: Arc<Window>,
  pub state: RenderState<'a>,
  resize_requested: bool,
}

impl<'a> Renderer<'a> {
  pub async fn new(window: Arc<Window>) -> Self {
    let state = RenderState::new(window.clone()).await;

    let shader = state.device.create_shader_module(include_wgsl!("../../../shaders/triangle.wgsl"));
    let layout = render_pipeline::create_layout(&state.device);
    let render_pipeline = render_pipeline::create_pipeline(&state.device, &layout, &shader, &state.config);

    Self {window, state, resize_requested: false}
  }

  pub fn begin_rendering(&mut self) -> Result<Option<FrameContext>, wgpu::SurfaceError> {
    self.state.resize(); 

    let output = self.state.surface.get_current_texture()?;
    let encoder = self
      .state.device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });
    
    Ok(Some(FrameContext::new(encoder, output, self.state.queue.clone())))
  }

  pub fn render_objects(&self, render_pass: &mut wgpu::RenderPass, objects: &Vec<RenderObject>) {
    for object in objects {
      render_pass.set_pipeline(&object.shader_pass.render_pipeline);
      render_pass.draw(0..3, 0..1);
    }
  }

  pub fn finish_rendering(&self, frame_ctx: FrameContext) {
    self.state.queue.submit(Some(frame_ctx.encoder.finish()));
    frame_ctx.output.present();
  }

  pub fn request_resize(&mut self, new_size: PhysicalSize<u32>) {
    self.state.new_size = Some(new_size);
  }


}