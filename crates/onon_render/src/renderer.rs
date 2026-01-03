use crate::{
  render_object::RenderObject,
  render_resource::{FrameContext, RenderState, render_pipeline},
};
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer<'a> {
  pub render_state: RenderState<'a>,
  pipeline_manager: render_pipeline::PipelineManager,
  bind_groups: Vec<wgpu::BindGroup>,
}

impl<'a> Renderer<'a> {
  pub async fn new(window: Arc<Window>) -> Self {
    let render_state = RenderState::new(window.clone()).await;
    let mut pipeline_manager = render_pipeline::PipelineManager::new();

    let solid_pipeline = render_pipeline::helpers::create_solid_pipeline(&render_state);
    pipeline_manager.add_pipeline(render_pipeline::PipelineType::Solid, solid_pipeline);

    let bind_groups = Vec::new();
    let texture_bind_group_layout =
      render_state
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
          entries: &[
            wgpu::BindGroupLayoutEntry {
              binding: 0,
              visibility: wgpu::ShaderStages::FRAGMENT,
              ty: wgpu::BindingType::Texture {
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
              },
              count: None,
            },
            wgpu::BindGroupLayoutEntry {
              binding: 1,
              visibility: wgpu::ShaderStages::FRAGMENT,
              ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
              count: None,
            },
          ],
          label: Some("texture_bind_group_layout"),
        });

    let diffuse_bind_group = render_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &texture_bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
        },
      ],
      label: Some("diffuse_bind_group"),
    });

    Self {
      render_state,
      pipeline_manager,
      bind_groups,
    }
  }

  pub fn begin_rendering(&mut self) -> Result<Option<FrameContext>, wgpu::SurfaceError> {
    self.render_state.resize();

    let output = self.render_state.surface.get_current_texture()?;
    let encoder =
      self
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
