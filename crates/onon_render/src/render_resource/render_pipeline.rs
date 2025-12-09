use wgpu::{
  Device, PipelineLayout, RenderPipeline, ShaderModule, SurfaceConfiguration
};

#[derive(Hash, Eq, PartialEq)]
pub enum PipelineType {
  Solid,
  Wireframe
}

pub fn create_layout(device: &Device) -> PipelineLayout {
  device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("Render Pipeline Layout"),
    bind_group_layouts: &[],
    push_constant_ranges: &[],
  })
}

pub fn create_pipeline(
  device: &Device,
  layout: &PipelineLayout,
  shader: &ShaderModule,
  config: &SurfaceConfiguration,
) -> RenderPipeline {
  device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(&layout),
    vertex: wgpu::VertexState {
      module: &shader,
      compilation_options: Default::default(),
      entry_point: Some("vs_main"),
      buffers: &[],
    },
    fragment: Some(wgpu::FragmentState {
      module: &shader,
      compilation_options: Default::default(),
      entry_point: Some("fs_main"),
      targets: &[Some(wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
      })],
    }),
    primitive: wgpu::PrimitiveState {
      topology: wgpu::PrimitiveTopology::TriangleList,
      strip_index_format: None,
      front_face: wgpu::FrontFace::Ccw,
      cull_mode: Some(wgpu::Face::Back),
      polygon_mode: wgpu::PolygonMode::Fill,
      unclipped_depth: false,
      conservative: false,
    },
    depth_stencil: None,
    multisample: wgpu::MultisampleState {
      count: 1,
      mask: !0,
      alpha_to_coverage_enabled: false,
    },
    multiview: None,
    cache: None,
  })
}
