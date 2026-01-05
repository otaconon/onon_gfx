use crate::render_resource;

pub fn create_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
  device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("Render Pipeline Layout"),
    bind_group_layouts: &[],
    immediate_size: 12,
  })
}

pub fn create_solid_pipeline(render_state: &render_resource::RenderState) -> wgpu::RenderPipeline {
  let shader = render_state
    .device()
    .create_shader_module(wgpu::include_wgsl!("../../../../../shaders/triangle.wgsl"));

  //let layout = create_layout(&render_state.device());
  let mut pipeline_builder = render_resource::render_pipeline::PipelineBuilder::new();
  pipeline_builder.add_target(render_state.config.format);
  pipeline_builder.set_vertex(&shader, "vs_main");
  pipeline_builder.set_fragment(&shader, "fs_main");
  pipeline_builder.create_pipeline(&render_state.device()).unwrap()
}