use winit::window::Window;
use std::sync::Arc;

pub fn create_canvas(window: Arc<Window>) {
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
