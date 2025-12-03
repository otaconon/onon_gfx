use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
  window::{Window, WindowId},
};

// 1. The State struct holds the GPU state and the Window
struct State {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  window: Arc<Window>,
}

impl State {
  // We use Arc<Window> so the State can hold a reference to the window
  // without lifetime issues.
  async fn new(window: Arc<Window>) -> Self {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });

    // Using Arc<Window> allows the surface to be 'static
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
        required_limits: wgpu::Limits::default(),
        label: None,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off
      })
      .await
      .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
      .formats
      .iter()
      .copied()
      .find(|f| f.is_srgb())
      .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
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
    }
  }

  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }

  fn update(&mut self) {
    // Update logic goes here
  }

  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
          depth_slice: None,
          view: &view,
          resolve_target: None,
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
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
      });
    }

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}

// 2. The App struct controls the application lifecycle
#[derive(Default)]
struct App {
  state: Option<State>,
}

impl ApplicationHandler for App {
  // This runs when the app starts up
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    // Create the window here
    let window = Arc::new(
      event_loop
        .create_window(Window::default_attributes())
        .unwrap(),
    );

    // Initialize wgpu State
    let state = pollster::block_on(State::new(window));
    self.state = Some(state);
  }

  // This runs for all window events (resize, close, etc.)
  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent,
  ) {
    if let Some(state) = &mut self.state {
      if window_id == state.window.id() {
        match event {
          WindowEvent::CloseRequested => {
            println!("The close button was pressed; stopping");
            event_loop.exit();
          }
          WindowEvent::Resized(physical_size) => {
            state.resize(physical_size);
          }
          WindowEvent::RedrawRequested => {
            state.update();
            match state.render() {
              Ok(_) => {}
              Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
              Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
              Err(e) => eprintln!("{:?}", e),
            }
          }
          _ => {}
        }
      }
    }
  }

  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    if let Some(state) = &self.state {
      state.window.request_redraw();
    }
  }
}

fn main() {
  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);

  let mut app = App::default();
  event_loop.run_app(&mut app).unwrap();
}
