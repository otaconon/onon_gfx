use wgpu::{Adapter, Device, Instance, Queue, Surface};

pub async fn query_adapter(instance: &Instance, surface: &Surface<'_>) -> Adapter {
  instance
    .request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      compatible_surface: Some(&surface),
      force_fallback_adapter: false,
    })
    .await
    .unwrap()
}

pub async fn query_device(adapter: &Adapter) -> (Device, Queue) {
  adapter
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
    .unwrap()
}