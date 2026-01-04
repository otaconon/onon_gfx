use crate::render_resource::TextureArray;

pub struct TextureArrayManager {
  texture_arrays: Vec<TextureArray>
}

impl TextureArrayManager {
  pub fn deafult() -> Self {
    Self {
      texture_arrays: Vec::new()
    }
  }

  pub fn add(&mut self, texture_array: TextureArray) {
    self.texture_arrays.push(texture_array);
  }
}