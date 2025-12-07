use std::sync::Arc;
use wgpu::include_wgsl;
use winit::window::Window;

pub struct WgpuApp {
  pub window: Arc<Window>,
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  size_changed: bool,
  clear_color: wgpu::Color,
  render_pipeline: wgpu::RenderPipeline,
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

    let shader = device.create_shader_module(include_wgsl!("../shaders/triangle.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        compilation_options: Default::default(),
        entry_point: Some("vs_main"),
        buffers: &[],
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        compilation_options: Default::default(),
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
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
    });

    Self {
      window,
      surface,
      device,
      queue,
      config,
      size,
      size_changed: false,
      clear_color: wgpu::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
      },
      render_pipeline,
    }
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
    match _delta {
      winit::event::MouseScrollDelta::PixelDelta(pos) => {
        let y = pos.y / 1000.0;
        self.clear_color.g = (self.clear_color.g + y).clamp(0.0, 1.0);
        self.clear_color.b = (self.clear_color.b + y).clamp(0.0, 1.0);
        log::info!(
          "clear color: {}, {}",
          self.clear_color.g,
          self.clear_color.b
        );
      }
      _ => {}
    };

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

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          depth_slice: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(self.clear_color),
            store: wgpu::StoreOp::Store,
          },
        })],
        ..Default::default()
      });
      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.draw(0..3, 0..1);
    }

    self.queue.submit(Some(encoder.finish()));
    output.present();

    Ok(())
  }
}
