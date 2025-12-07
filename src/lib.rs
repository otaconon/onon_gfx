pub mod utils;
use onon_internals::app;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub fn run() -> anyhow::Result<()> {
  #[cfg(not(target_arch = "wasm32"))]
  {
    env_logger::init();
  }
  #[cfg(target_arch = "wasm32")]
  {
    console_log::init_with_level(log::Level::Info).unwrap();
  }

  let event_loop = winit::event_loop::EventLoop::with_user_event().build()?;
  let mut app = app::handler::WgpuAppHandler::default();
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
