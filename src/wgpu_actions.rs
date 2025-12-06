use std::sync::Arc;
use wgpu::WasmNotSend;
use winit::{
  window::{Window},
  dpi::{PhysicalSize, PhysicalPosition},
  event::*
};

pub trait WgpuAppAction {
  fn new(window: Arc<Window>) -> impl core::future::Future<Output = Self> + WasmNotSend;
  fn set_window_resized(&mut self, new_size: PhysicalSize<u32>);
  fn get_size(&self) -> PhysicalSize<u32>;
  fn keyboard_input(&mut self, _event: &KeyEvent) -> bool;
  fn mouse_click(&mut self, _state: ElementState, _button: MouseButton) -> bool;
  fn mouse_wheel(&mut self, _delta: MouseScrollDelta, _phase: TouchPhase) -> bool;
  fn cursor_move(&mut self, _position: PhysicalPosition<f64>) -> bool;
  fn device_input(&mut self, _event: &DeviceEvent) -> bool;
  fn update(&mut self, _dt: web_time::Duration) {}
  fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}