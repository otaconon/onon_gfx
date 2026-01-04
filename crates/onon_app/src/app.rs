use onon_render::{RenderObject, Renderer, TextureArrayManager, render_resource::TextureArray};
use std::sync::Arc;
use winit::window::Window;

pub struct WgpuApp {
  pub window: Arc<Window>,
  renderer: Renderer<'static>,
  objects: Vec<RenderObject>,
}

const VERTICES: &[onon_render::mesh::Vertex] = &[
  onon_render::mesh::Vertex {
    position: [0.0, 0.5],
    color: [1.0, 0.0, 0.0],
  },
  onon_render::mesh::Vertex {
    position: [-0.5, -0.5],
    color: [0.0, 1.0, 0.0],
  },
  onon_render::mesh::Vertex {
    position: [0.5, -0.5],
    color: [0.0, 0.0, 1.0],
  },
];

const INDICES: &[u16] = &[0, 1, 2, 0];

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
    ));

    let mesh = onon_render::mesh::Mesh2D::new(
      VERTICES.to_vec(),
      INDICES.to_vec(),
      &renderer.render_state.device(),
    );
    let render_objects = vec![RenderObject::new(mesh, 0)];

    Self {
      window: window.clone(),
      renderer: renderer,
      objects: render_objects,
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
