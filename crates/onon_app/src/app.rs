use onon_render::{RenderObject, Renderer, render_resource::render_pipeline::PipelineType};
use std::sync::Arc;
use winit::{
  event::ElementState, keyboard::{Key, NamedKey}, window::Window
};

pub struct WgpuApp {
  pub window: Arc<Window>,
  renderer: Renderer<'static>,
  objects: Vec<RenderObject>,
  pipelines: Vec<PipelineType>,
  current_pipeline: usize,
}

impl WgpuApp {
  pub async fn new(window: Arc<Window>) -> Self {
    #[cfg(target_arch = "wasm32")]
    {
      use crate::canvas::create_canvas;
      create_canvas(window.clone());
    }

    let renderer = Renderer::new(window.clone()).await;
    let render_objects = Vec::new();

    Self {
      window: window.clone(),
      renderer: renderer,
      objects: render_objects,
      pipelines: vec![PipelineType::Solid, PipelineType::Triangle],
      current_pipeline: 0,
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
          let res = self.renderer.render(
            &mut render_pass,
            self.pipelines[self.current_pipeline].clone(),
          );
          match res {
            Ok(()) => {}
            Err(e) => log::error!("{}", e),
          }
        }
        self.renderer.finish_rendering(frame_ctx);
      }
      Ok(None) => {}
      Err(wgpu::SurfaceError::Lost) => eprintln!("Surface is lost"),
      Err(e) => eprintln!("{e:?}"),
    }
    self.window.request_redraw();
  }

  pub fn keyboard_input(&mut self, event: &winit::event::KeyEvent) -> bool {
    if event.state == ElementState::Pressed && !event.repeat {
      match event.logical_key {
        Key::Named(NamedKey::Space) => {
          self.current_pipeline = (self.current_pipeline + 1) % self.pipelines.len();
        }
        _ => {}
      }
    }
    true
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
    if new_size == self.renderer.state.get_size() {
      return;
    }
    self.renderer.request_resize(new_size);
  }
}
