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

  pub fn get_texture_array(&self, index: u32) -> Option<&TextureArray> {
    self.texture_arrays.get(index as usize)
  }


  pub fn get_texture_array_mut(&mut self, index: u32) -> Option<&mut TextureArray> {
    self.texture_arrays.get_mut(index as usize)
  }
}