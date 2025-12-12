use crate::{
  render_object::RenderObject,
  render_resource::{
    FrameContext, RenderState,
    render_pipeline,
  },
};
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer<'a> {
  pub render_state: RenderState<'a>,
  pipeline_manager: render_pipeline::PipelineManager,
}

impl<'a> Renderer<'a> {
  pub async fn new(window: Arc<Window>) -> Self {
    let render_state = RenderState::new(window.clone()).await;
    let mut pipeline_manager = render_pipeline::PipelineManager::new();

    let solid_pipeline = render_pipeline::helpers::create_solid_pipeline(&render_state);
    pipeline_manager.add_pipeline(render_pipeline::PipelineType::Solid, solid_pipeline);

    Self {
      render_state,
      pipeline_manager
    }
  }

  pub fn begin_rendering(&mut self) -> Result<Option<FrameContext>, wgpu::SurfaceError> {
    self.render_state.resize();

    let output = self.render_state.surface.get_current_texture()?;
    let encoder = self
      .render_state
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    Ok(Some(FrameContext::new(
      encoder,
      output,
      self.render_state.queue.clone(),
    )))
  }

  pub fn render_solids(
    &self,
    render_pass: &mut wgpu::RenderPass,
    objects: &Vec<RenderObject>,
  ) -> Result<(), &'static str> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(render_pipeline::PipelineType::Solid)
      .ok_or("No solid pipeline is setup")?;
    render_pass.set_pipeline(pipeline);

    for object in objects {
      render_pass.draw(0..3, 0..1);
    }

    render_pass.draw(0..3, 0..1);

    Ok(())
  }

  #[allow(unused)]
  pub fn render_wireframes(&self, render_pass: &mut wgpu::RenderPass) -> Result<(), &'static str> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(render_pipeline::PipelineType::Wireframe)
      .ok_or("No wireframe pipeline is setup")?;
    render_pass.set_pipeline(pipeline);

    Ok(())
  }

  pub fn finish_rendering(&self, frame_ctx: FrameContext) {
    self.render_state.queue.submit(Some(frame_ctx.encoder.finish()));
    frame_ctx.output.present();
  }

  pub fn request_resize(&mut self, new_size: PhysicalSize<u32>) {
    self.render_state.new_size = Some(new_size);
  }
}
