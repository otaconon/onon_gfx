use crate::{
  TextureArrayManager, render_object::RenderObject, render_resource::{FrameContext, RenderState, render_pipeline}, texture_array_manager
};
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer<'a> {
  pub render_state: RenderState<'a>,
  pipeline_manager: render_pipeline::PipelineManager,
  pub texture_array_bind_group_layout: wgpu::BindGroupLayout // TODO: Remove from here
}

impl<'a> Renderer<'a> {
  pub async fn new(window: Arc<Window>) -> Self {
    let render_state = RenderState::new(window.clone()).await;
    let mut pipeline_manager = render_pipeline::PipelineManager::new();

    let solid_pipeline = render_pipeline::helpers::create_solid_pipeline(&render_state);
    let texture_array_bind_group_layout = solid_pipeline.get_bind_group_layout(0);
    pipeline_manager.add_pipeline(render_pipeline::PipelineType::Solid, solid_pipeline);

    Self {
      render_state,
      pipeline_manager,
      texture_array_bind_group_layout
    }
  }

  pub fn begin_rendering(&mut self) -> Result<Option<FrameContext>, wgpu::SurfaceError> {
    self.render_state.resize();

    let output = self.render_state.surface.get_current_texture()?;
    let encoder =
      self
        .render_state
        .device()
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
    texture_array_manager: &TextureArrayManager
  ) -> Result<(), &'static str> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(render_pipeline::PipelineType::Solid)
      .ok_or("No solid pipeline is setup")?;

    render_pass.set_pipeline(pipeline);

    for object in objects {
      let texture_array = texture_array_manager.get_texture_array(object.texture_array_id).unwrap();
      render_pass.set_bind_group(0, &texture_array.bind_group, &[]);
      render_pass.set_vertex_buffer(0, object.mesh.vertex_buffer.slice(..));
      render_pass.set_index_buffer(
        object.mesh.index_buffer.slice(..),
        wgpu::IndexFormat::Uint16,
      );
      render_pass.draw_indexed(0..object.mesh.indices.len() as u32, 0, 0..1);
    }

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
    self
      .render_state
      .queue
      .submit(Some(frame_ctx.encoder.finish()));
    frame_ctx.output.present();
  }

  pub fn request_resize(&mut self, new_size: PhysicalSize<u32>) {
    self.render_state.new_size = Some(new_size);
  }
}
