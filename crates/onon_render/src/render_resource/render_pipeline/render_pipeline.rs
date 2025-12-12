
#[derive(Hash, Eq, PartialEq)]
pub enum PipelineType {
  Solid,
  Wireframe,
}

#[derive(Clone)]
pub struct PipelineBuilder<'a> {
  layout: Option<&'a wgpu::PipelineLayout>,
  cull_mode: wgpu::Face,
  polygon_mode: wgpu::PolygonMode,
  targets: Vec<Option<wgpu::ColorTargetState>>,
  vertex_module: Option<&'a wgpu::ShaderModule>,
  vertex_entry: &'a str,
  fragment_module: Option<&'a wgpu::ShaderModule>,
  fragment_entry: &'a str,
}

impl<'a> PipelineBuilder<'a> {
  pub fn new() -> Self {
    Self {
      layout: None,
      cull_mode: wgpu::Face::Back,
      polygon_mode: wgpu::PolygonMode::Fill,
      vertex_module: None,
      vertex_entry: "",
      fragment_module: None,
      fragment_entry: "",
      targets: Vec::new(),
    }
  }

  pub fn create_pipeline(
    self,
    device: &wgpu::Device,
  ) -> Result<wgpu::RenderPipeline, &'static str> {

    let vertex_module = self.vertex_module.as_ref().ok_or("Vertex shader hasn't been specified")?;
    let vertex = wgpu::VertexState {
      module: &vertex_module,
      compilation_options: Default::default(),
      entry_point: Some("vs_main"),
      buffers: &[],
    };

    let fragment = match self.fragment_module.as_ref() {
      Some(shader_module) => Some(wgpu::FragmentState {
        module: shader_module,
        compilation_options: Default::default(),
        entry_point: Some("fs_main"),
        targets: &self.targets,
      }),
      None => None
    };

    Ok(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: self.layout,
      vertex: vertex,
      fragment: fragment,
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(self.cull_mode),
        polygon_mode: self.polygon_mode,
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
    }))
  }

  pub fn set_layout(&mut self, layout: &'a wgpu::PipelineLayout) {
    self.layout = Some(layout);
  }

  pub fn set_vertex(&mut self, shader_module: &'a wgpu::ShaderModule, entry_point: &'a str) {
    self.vertex_module = Some(shader_module);
    self.vertex_entry = entry_point;
  }

  pub fn set_fragment(&mut self, shader_module: &'a wgpu::ShaderModule, entry_point: &'a str) {
    self.fragment_module = Some(shader_module);
    self.fragment_entry = entry_point;
  }

  pub fn set_cull_mode(&mut self, cull_face: wgpu::Face) {
    self.cull_mode = cull_face;
  }

  pub fn set_polygon_mode(&mut self, mode: wgpu::PolygonMode) {
    self.polygon_mode = mode;
  }

  pub fn add_target(&mut self, format: wgpu::TextureFormat) {
    self.targets.push(Some(wgpu::ColorTargetState {
      format: format,
      blend: Some(wgpu::BlendState::REPLACE),
      write_mask: wgpu::ColorWrites::ALL,
    }))
  }
}