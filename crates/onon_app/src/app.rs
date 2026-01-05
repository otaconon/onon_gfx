use onon_render::{RenderObject, Renderer, TextureArrayManager, mesh::Vertex, render_resource::TextureArray};
use std::sync::Arc;
use winit::window::Window;

pub struct WgpuApp {
  pub window: Arc<Window>,
  renderer: Renderer<'static>,
  objects: Vec<RenderObject>,
  texture_array_manager: TextureArrayManager
}

const VERTICES: &[onon_render::mesh::Vertex] = &[
  Vertex { position: [-0.0868241, 0.49240386], tex_coords: [0.4131759, 0.00759614], }, // A
  Vertex { position: [-0.49513406, 0.06958647], tex_coords: [0.0048659444, 0.43041354], }, // B
  Vertex { position: [-0.21918549, -0.44939706], tex_coords: [0.28081453, 0.949397], }, // C
  Vertex { position: [0.35966998, -0.3473291], tex_coords: [0.85967, 0.84732914], }, // D
  Vertex { position: [0.44147372, 0.2347359], tex_coords: [0.9414737, 0.2652641], }, // E
];

const INDICES: &[u16] = &[
  0, 1, 2, 
  0, 2, 3, 
  0, 3, 4
];

impl WgpuApp {
  pub async fn new(window: Arc<Window>) -> Self {
    #[cfg(target_arch = "wasm32")]
    {
      use crate::canvas::create_canvas;
      create_canvas(window.clone());
    }

    let renderer = Renderer::new(window.clone()).await;

    let diffuse_sampler = Arc::new(renderer.render_state.device().create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::MipmapFilterMode::Nearest,
      ..Default::default()
    }));

    let mut texture_array_manager = TextureArrayManager::deafult();
    texture_array_manager.add(TextureArray::srgba8_texture(
      renderer.render_state.device(),
      wgpu::Extent3d {
        width: 256,
        height: 256,
        depth_or_array_layers: 5,
      },
      diffuse_sampler,
      &renderer.texture_array_bind_group_layout
    ));

    let mesh = onon_render::mesh::Mesh2D::new(
      VERTICES.to_vec(),
      INDICES.to_vec(),
      &renderer.render_state.device(),
    );
    
    let diffuse_bytes = include_bytes!("../../../resources/happy-tree-cartoon.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.to_rgba8();

    use image::GenericImageView;
    let dimensions = diffuse_image.dimensions();
    let slot: u32;
    match texture_array_manager.get_texture_array_mut(0) {
      Some(texture_array) => {
        slot = texture_array.upload_texture(&renderer.render_state.queue, &diffuse_rgba, dimensions.0, dimensions.1);
      }
      None => {
        slot = 0;
        log::error!("Failed to get texture");
      }
    }

    let render_objects = vec![RenderObject::new(mesh, 0, 0, slot)];

    Self {
      window: window.clone(),
      renderer: renderer,
      objects: render_objects,
      texture_array_manager
    }
  }

  pub fn redraw(&mut self) {
    self.window.pre_present_notify();

    match self.renderer.begin_rendering() {
      Ok(Some(mut frame_ctx)) => {
        let view = frame_ctx
          .output
          .texture
          .create_view(&wgpu::TextureViewDescriptor::default());
        {
          let mut render_pass = frame_ctx.create_render_pass(&view);
          let res = self.renderer.render_solids(&mut render_pass, &self.objects, &self.texture_array_manager);
          match res {
            Ok(()) => {}
            Err(e) => log::error!("{}", e),
          }
        }
        self.renderer.finish_rendering(frame_ctx);
      }
      Ok(None) => {}
      Err(wgpu::SurfaceError::Lost) => log::error!("Surface is lost"),
      Err(e) => log::error!("{e:?}"),
    }
    self.window.request_redraw();
  }

  pub fn keyboard_input(&mut self, _event: &winit::event::KeyEvent) -> bool {
    false
  }

  pub fn mouse_click(
    &mut self,
    _state: winit::event::ElementState,
    _button: winit::event::MouseButton,
  ) -> bool {
    false
  }

  pub fn mouse_wheel(
    &mut self,
    _delta: winit::event::MouseScrollDelta,
    _phase: winit::event::TouchPhase,
  ) -> bool {
    true
  }

  pub fn cursor_move(&mut self, _position: winit::dpi::PhysicalPosition<f64>) -> bool {
    false
  }

  #[allow(unused)]
  pub fn device_input(&mut self, _event: &winit::event::DeviceEvent) -> bool {
    false
  }

  pub fn set_window_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size == self.renderer.render_state.get_size() {
      return;
    }
    self.renderer.request_resize(new_size);
  }
}
