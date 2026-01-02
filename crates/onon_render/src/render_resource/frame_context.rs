pub struct FrameContext {
  pub encoder: wgpu::CommandEncoder,
  pub output: wgpu::SurfaceTexture,
  queue: wgpu::Queue,
}

impl FrameContext {
  pub fn new(encoder: wgpu::CommandEncoder, output: wgpu::SurfaceTexture, queue: wgpu::Queue) -> Self{
    Self { encoder, output, queue }
  }

  pub fn present(self) {
    self.queue.submit(Some(self.encoder.finish()));
    self.output.present();
  }

  pub fn create_render_pass<'a>(&'a mut self, view: &'a wgpu::TextureView) -> wgpu::RenderPass<'a> {
    self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("Render Pass"),
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
          store: wgpu::StoreOp::Store,
        },
        depth_slice: None,
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
      multiview_mask: None
    })
  }
}
