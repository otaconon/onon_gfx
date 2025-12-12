use crate::render_resource::render_pipeline::PipelineType;
use std::collections::HashMap;


pub struct PipelineManager {
  pipelines: HashMap<PipelineType, wgpu::RenderPipeline>
}

impl PipelineManager {
  pub fn new() -> Self {
    Self {pipelines: HashMap::new()}
  }

  pub fn add_pipeline(&mut self, pipeline_type: PipelineType, pipeline: wgpu::RenderPipeline) {
    self.pipelines.insert(pipeline_type, pipeline);
  }

  pub fn get_pipeline(&self, pipeline_type: PipelineType) -> Option<&wgpu::RenderPipeline> {
    self.pipelines.get(&pipeline_type)
  }
}