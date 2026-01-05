use crate::{
  TextureManager,
  render_object::RenderObject,
  render_resource::{FrameContext, RenderState, render_pipeline, texture_array::TextureArrayInfo},
  texture_manager,
};
use anyhow::{Context, Result};
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer<'a> {
  pub render_state: RenderState<'a>,
  pipeline_manager: render_pipeline::PipelineManager,
  texture_manager: TextureManager,
}

impl<'a> Renderer<'a> {
  pub async fn new(window: Arc<Window>) -> Self {
    let render_state = RenderState::new(window.clone()).await;
    let mut pipeline_manager = render_pipeline::PipelineManager::new();

    let solid_pipeline = render_pipeline::helpers::create_solid_pipeline(&render_state);

    let diffuse_sampler = Arc::new(render_state.device().create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::MipmapFilterMode::Nearest,
      ..Default::default()
    }));

    let mut texture_manager = TextureManager::deafult();
    let texture_array_info = TextureArrayInfo {
      dims: wgpu::Extent3d {
        width: 256,
        height: 256,
        depth_or_array_layers: 5,
      },
      sampler: diffuse_sampler,
      bind_group_layout: solid_pipeline.get_bind_group_layout(0)
    };
    texture_manager.add_texture_array(render_state.device(), texture_array_info);

    pipeline_manager.add_pipeline(render_pipeline::PipelineType::Solid, solid_pipeline);

    Self {
      render_state,
      pipeline_manager,
      texture_manager,
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
    objects: &Vec<RenderObject>
  ) -> Result<()> {
    let pipeline = self
      .pipeline_manager
      .get_pipeline(render_pipeline::PipelineType::Solid)
      .context("No solid pipeline is setup")?;

    render_pass.set_pipeline(pipeline);

    for object in objects {
      let texture_array = self.texture_manager
        .get_texture_array(&object.texture_array_info)
        .context("Failed to get texture array")?;

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
