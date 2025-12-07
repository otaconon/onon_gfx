use onon_render::Renderer;
use std::sync::Arc;
use winit::window::Window;

pub struct WgpuApp {
  pub window: Arc<Window>,
  renderer: Renderer,
}

impl WgpuApp {
  pub async fn new(window: Arc<Window>) -> Self {
    #[cfg(target_arch = "wasm32")]
    {
      use winit::platform::web::WindowExtWebSys;

      let canvas = window.canvas().unwrap();

      web_sys::window()
        .and_then(|win| win.document())
        .map(|doc| {
          let _ = canvas.set_attribute("id", "winit-canvas");
          match doc.get_element_by_id("wgpu-app-container") {
            Some(dst) => {
              let _ = dst.append_child(canvas.as_ref());
            }
            None => {
              let container = doc.create_element("div").unwrap();
              let _ = container.set_attribute("id", "wgpu-app-container");
              let _ = container.append_child(canvas.as_ref());

              doc.body().map(|body| body.append_child(container.as_ref()));
            }
          };
        })
        .expect("Failed to create canvas");

      canvas.set_tab_index(0);

      let style = canvas.style();
      style.set_property("outline", "none").unwrap();
      canvas.focus().expect("Can't focus on canvas");
    }

    Self {
      window: window.clone(),
      renderer: Renderer::new(window).await,
    }
  }

  pub fn redraw(&mut self) {
    self.window.pre_present_notify();

    match self.renderer.render() {
      Ok(_) => {}
      Err(wgpu::SurfaceError::Lost) => eprintln!("Surface is lost"),
      Err(e) => eprintln!("{e:?}"),
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
    if new_size == self.renderer.size {
      return;
    }
    self.renderer.size = new_size;
    self.renderer.request_resize();
  }
}
