use crate::render_resource::ShaderEffect;

pub struct ShaderPass {
  pub render_pipeline: wgpu::RenderPipeline,
  shader_effect: ShaderEffect
}