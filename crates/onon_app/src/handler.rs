use crate::app::WgpuApp;
use parking_lot::Mutex;
use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::ActiveEventLoop,
  window::{Window, WindowId},
};

#[derive(Default)]
pub struct WgpuAppHandler {
  app: Arc<Mutex<Option<WgpuApp>>>,
  #[allow(dead_code)]
  missed_resize: Arc<Mutex<Option<winit::dpi::PhysicalSize<u32>>>>,
}

impl ApplicationHandler for WgpuAppHandler {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.app.as_ref().lock().is_some() {
      return;
    }

    let window_attributes = Window::default_attributes().with_title("tutorial1-window");
    let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

    cfg_if::cfg_if! {
      if #[cfg(target_arch = "wasm32")] {
        let app = self.app.clone();
        let missed_resize = self.missed_resize.clone();

        wasm_bindgen_futures::spawn_local(async move {
          let window_cloned = window.clone();

          let wgpu_app = WgpuApp::new(window).await;
          let mut app = app.lock();
          *app = Some(wgpu_app);

          if let Some(resize) = *missed_resize.lock() {
            app.as_mut().unwrap().set_window_resized(resize);
          }
          window_cloned.request_redraw();
        });
      } else {
        let wgpu_app = pollster::block_on(WgpuApp::new(window));
        self.app.lock().replace(wgpu_app);
      }
    }
  }

  fn suspended(&mut self, _event_loop: &ActiveEventLoop) {}

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    let mut app = self.app.lock();
    if app.as_ref().is_none() {
      if let WindowEvent::Resized(physical_size) = event
        && physical_size.width > 0
        && physical_size.height > 0
      {
        let mut missed_resize = self.missed_resize.lock();
        *missed_resize = Some(physical_size);
      }
      return;
    }

    let app = app.as_mut().unwrap();
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      WindowEvent::Resized(physical_size) => {
        if physical_size.width == 0 || physical_size.height == 0 {
        } else {
          app.set_window_resized(physical_size);
        }
      }
      WindowEvent::KeyboardInput { event, .. } => {
        app.keyboard_input(&event);
      }
      WindowEvent::CursorMoved { position, .. } => {
        app.cursor_move(position);
      }
      WindowEvent::MouseInput { state, button, .. } => {
        app.mouse_click(state, button);
      }
      WindowEvent::MouseWheel { delta, phase, .. } => {
        app.mouse_wheel(delta, phase);
      }
      WindowEvent::RedrawRequested => {
        app.redraw(); 
      }
      _ => (),
    }
  }
}
