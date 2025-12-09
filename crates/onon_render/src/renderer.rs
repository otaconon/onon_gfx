use crate::{
  PipelineManager,
  render_object::RenderObject,
  render_resource::{
    FrameContext, RenderState,
    render_pipeline::{self, PipelineType},
  },
};
use std::sync::Arc;
use wgpu::{include_wgsl};
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer<'a> {
  pub state: RenderState<'a>,
  pipeline_manager: PipelineManager,
}

impl<'a> Renderer<'a> {
  pub async fn new(window: Arc<Window>) -> Self {
    let state = RenderState::new(window.clone()).await;
    let mut pipeline_manager = PipelineManager::new();

    let shader = state
      .device
      .create_shader_module(include_wgsl!("../../../shaders/solid.wgsl"));
    let layout = render_pipeline::create_layout(&state.device);
    let pipeline = render_pipeline::create_pipeline(&state.device, &layout, &shader, &state.config);
    pipeline_manager.add_pipeline(PipelineType::Solid, pipeline);

    let shader = state
      .device
      .create_shader_module(include_wgsl!("../../../shaders/triangle.wgsl"));
    let layout = render_pipeline::create_layout(&state.device);
    let pipeline = render_pipeline::create_pipeline(&state.device, &layout, &shader, &state.config);
    pipeline_manager.add_pipeline(PipelineType::Triangle, pipeline);

    Self {
      state,
      pipeline_manager
    }
  }

  pub fn begin_rendering(&mut self) -> Result<Option<FrameContext>, wgpu::SurfaceError> {
    self.state.resize();

    let output = self.state.surface.get_current_texture()?;
    let encoder = self
      .state
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    Ok(Some(FrameContext::new(
      encoder,
      output,
      self.state.queue.clone(),
    )))
  }

  pub fn render(&self, render_pass: &mut wgpu::RenderPass, pipeline_type: PipelineType) -> Result<(), &'static str> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(pipeline_type)
      .ok_or("No triangle pipeline is setup")?;

    render_pass.set_pipeline(pipeline);
    render_pass.draw(0..3, 0..1);

    Ok(())
  }

  pub fn render_solids(
    &self,
    render_pass: &mut wgpu::RenderPass,
    objects: &Vec<RenderObject>,
  ) -> Result<(), &'static str> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(PipelineType::Solid)
      .ok_or("No solid pipeline is setup")?;
    render_pass.set_pipeline(pipeline);

    for object in objects {
      render_pass.draw(0..3, 0..1);
    }

    render_pass.draw(0..3, 0..1);

    Ok(())
  }

  pub fn render_triangle(
    &self,
    render_pass: &mut wgpu::RenderPass,
  ) -> Result<(), &'static str> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(PipelineType::Triangle)
      .ok_or("No triangle pipeline is setup")?;

    render_pass.set_pipeline(pipeline);
    render_pass.draw(0..3, 0..1);

    Ok(())
  }

  #[allow(unused)]
  pub fn render_wireframes(&self, render_pass: &mut wgpu::RenderPass) -> Result<(), &'static str> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(PipelineType::Wireframe)
      .ok_or("No wireframe pipeline is setup")?;
    render_pass.set_pipeline(pipeline);

    Ok(())
  }

  pub fn finish_rendering(&self, frame_ctx: FrameContext) {
    self.state.queue.submit(Some(frame_ctx.encoder.finish()));
    frame_ctx.output.present();
  }

  pub fn request_resize(&mut self, new_size: PhysicalSize<u32>) {
    self.state.new_size = Some(new_size);
  }
}
