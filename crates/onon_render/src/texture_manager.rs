use std::collections::HashMap;
use anyhow::{Result, Context};

use crate::render_resource::{TextureArray, texture_array::TextureArrayInfo};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TextureId(pub u32);

pub struct TextureManager {
  texture_arrays: HashMap<TextureArrayInfo, TextureArray>,
}

impl TextureManager {
  pub fn deafult() -> Self {
    Self {
      texture_arrays: HashMap::new(),
    }
  }

  pub fn add_texture_array(&mut self, device: &wgpu::Device, texture_array_info: TextureArrayInfo) {
    self.texture_arrays.insert(
      texture_array_info.clone(),
      TextureArray::new(device, &texture_array_info),
    );
  }

  pub fn get_texture_array(&self, texture_array_info: &TextureArrayInfo) -> Option<&TextureArray> {
    self.texture_arrays.get(texture_array_info)
  }

  pub fn get_texture_array_mut(
    &mut self,
    texture_array_info: &TextureArrayInfo,
  ) -> Option<&mut TextureArray> {
    self.texture_arrays.get_mut(texture_array_info)
  }

  pub fn add_texture<P: AsRef<std::path::Path>>(
    &mut self,
    queue: &wgpu::Queue,
    path: P,
    texture_array_info: &TextureArrayInfo,
  ) -> Result<u32> {
    let texture_array = self.get_texture_array_mut(texture_array_info).context("Failed to retrieve texture array")?;
    texture_array.load_from_file(queue, path)
  }
}
