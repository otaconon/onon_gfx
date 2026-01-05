use onon_render::{RenderObject, Renderer, mesh::Vertex};
use std::sync::Arc;
use winit::window::Window;

pub struct WgpuApp {
  pub window: Arc<Window>,
  renderer: Renderer<'static>,
  objects: Vec<RenderObject>
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

    let mesh = onon_render::mesh::Mesh2D::new(
      VERTICES.to_vec(),
      INDICES.to_vec(),
      &renderer.render_state.device(),
    );
    
    let render_objects = vec![RenderObject::new(mesh, 0, 0, slot)];

    Self {
      window: window.clone(),
      renderer: renderer,
      objects: render_objects
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
          let res = self.renderer.render_solids(&mut render_pass, &self.objects);
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
