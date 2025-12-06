use parking_lot::Mutex;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{ActiveEventLoop, EventLoop},
  window::{Window, WindowId},
};

pub mod utils;
mod wgpu_actions;

struct WgpuApp {
  window: Arc<Window>,
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  size_changed: bool,
}

impl WgpuApp {
  async fn new(window: Arc<Window>) -> Self {
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

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });
    let surface = instance.create_surface(window.clone()).unwrap();

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();

    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        required_features: wgpu::Features::empty(),
        required_limits: if cfg!(target_arch = "wasm32") {
          wgpu::Limits::downlevel_webgl2_defaults()
        } else {
          wgpu::Limits::default()
        },
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        label: None,
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
      })
      .await
      .unwrap();

    let caps = surface.get_capabilities(&adapter);
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: caps.formats[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    Self {
      window,
      surface,
      device,
      queue,
      config,
      size,
      size_changed: false,
    }
  }

  fn keyboard_input(&mut self, _event: &winit::event::KeyEvent) -> bool {
    false
  }

  fn mouse_click(
    &mut self,
    _state: winit::event::ElementState,
    _button: winit::event::MouseButton,
  ) -> bool {
    false
  }

  fn mouse_wheel(
    &mut self,
    _delta: winit::event::MouseScrollDelta,
    _phase: winit::event::TouchPhase,
  ) -> bool {
    false
  }

  fn cursor_move(&mut self, _position: winit::dpi::PhysicalPosition<f64>) -> bool {
    false
  }

  fn device_input(&mut self, _event: &winit::event::DeviceEvent) -> bool {
    false
  }

  fn set_window_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size == self.size {
      return;
    }
    self.size = new_size;
    self.size_changed = true;
  }

  fn resize_surface_if_needed(&mut self) {
    if self.size_changed {
      self.config.width = self.size.width;
      self.config.height = self.size.height;
      self.surface.configure(&self.device, &self.config);
      self.size_changed = false;
    }
  }

  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    if self.size.width == 0 || self.size.height == 0 {
      return Ok(());
    }
    self.resize_surface_if_needed();

    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });
    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          depth_slice: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            }),
            store: wgpu::StoreOp::Store,
          },
        })],
        ..Default::default()
      });
    }

    self.queue.submit(Some(encoder.finish()));
    output.present();

    Ok(())
  }
}

#[derive(Default)]
struct WgpuAppHandler {
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
      WindowEvent::KeyboardInput { .. } => {}
      WindowEvent::RedrawRequested => {
        app.window.pre_present_notify();

        match app.render() {
          Ok(_) => {}
          Err(wgpu::SurfaceError::Lost) => eprintln!("Surface is lost"),
          Err(e) => eprintln!("{e:?}"),
        }
        app.window.request_redraw();
      }
      _ => (),
    }
  }
}

pub fn run() -> anyhow::Result<()> {
  #[cfg(not(target_arch = "wasm32"))]
  {
    env_logger::init();
  }
  #[cfg(target_arch = "wasm32")]
  {
    console_log::init_with_level(log::Level::Info).unwrap();
  }

  let event_loop = EventLoop::with_user_event().build()?;
  let mut app = WgpuAppHandler::default();
  event_loop.run_app(&mut app)?;

  Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
  console_error_panic_hook::set_once();
  run().unwrap();

  Ok(())
}