use std::{cmp::Ordering};

use anyhow::{Context, Result, anyhow};
use naga::{
  AddressSpace, GlobalVariable, ImageClass, ImageDimension, Module, StorageAccess, StorageFormat, Type, TypeInner
};
use wgpu::{
  BindingType, BufferBindingType,
  SamplerBindingType, StorageTextureAccess, TextureSampleType,
  TextureViewDimension,
};

#[derive(Debug, Eq, PartialEq)]
pub struct ShaderBindingInfo {
  pub group: u32,
  pub binding: u32,
  pub ty: BindingType,
}

impl Ord for ShaderBindingInfo {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .group
      .cmp(&other.group)
      .then(self.binding.cmp(&other.binding))
  }
}

impl PartialOrd for ShaderBindingInfo {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

/// Holds bindings sorted by group first, binding second
pub struct Shader {
  module: Module,
  pub bindings: Vec<ShaderBindingInfo>,
}

impl Shader {
  pub fn new(module: Module) -> Self {
    let mut bindings: Vec<ShaderBindingInfo> = Vec::new();

    for (_, global) in module.global_variables.iter() {
      let ty = &module.types[global.ty];

      if let Some(ref binding) = global.binding {
        match get_binding_type(ty, global) {
          Ok(binding_type) => bindings.push(ShaderBindingInfo {
            group: binding.group,
            binding: binding.binding,
            ty: binding_type,
          }),
          Err(e) => log::error!("{}", e),
        }
      }
    }

    bindings.sort();
    Shader { module, bindings }
  }
}

fn get_binding_type(ty: &Type, global: &GlobalVariable) -> Result<BindingType> {
  let buffer_binding_type = match global.space {
    AddressSpace::Uniform => Some(BufferBindingType::Uniform),
    AddressSpace::Storage { access } => Some(BufferBindingType::Storage {
      read_only: !access.contains(StorageAccess::STORE),
    }),
    _ => None,
  };

  return match ty.inner {
    TypeInner::Struct { .. } => Ok(BindingType::Buffer {
      ty: buffer_binding_type.context("Buffer bindings type is: None")?,
      has_dynamic_offset: false,
      min_binding_size: None,
    }),
    TypeInner::Image {
      dim,
      arrayed,
      class,
    } => get_image_type(dim, arrayed, class),
    TypeInner::Sampler { .. } => Ok(BindingType::Sampler(SamplerBindingType::Filtering)),
    _ => return Err(anyhow!("Unhandled binding type")),
  };
}

fn get_image_type(dim: ImageDimension, arrayed: bool, class: ImageClass) -> Result<BindingType> {
  let view_dimension = match (dim, arrayed) {
    (ImageDimension::D2, false) => TextureViewDimension::D2,
    (ImageDimension::D2, true) => TextureViewDimension::D2Array,
    (ImageDimension::Cube, false) => TextureViewDimension::Cube,
    (ImageDimension::Cube, true) => TextureViewDimension::CubeArray,
    (ImageDimension::D3, false) => TextureViewDimension::D3,
    _ => TextureViewDimension::D2,
  };

  match class {
    ImageClass::Storage { format, access } => Ok(BindingType::StorageTexture {
      access: if access.contains(naga::StorageAccess::STORE | naga::StorageAccess::LOAD) {
        StorageTextureAccess::ReadWrite
      } else if access.contains(naga::StorageAccess::STORE) {
        StorageTextureAccess::WriteOnly
      } else {
        StorageTextureAccess::ReadOnly
      },
      format: match format {
        StorageFormat::R8Unorm => wgpu::TextureFormat::R8Unorm,
        StorageFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        StorageFormat::Rgba8Snorm => wgpu::TextureFormat::Rgba8Snorm,
        _ => panic!("Unsupported storage format"),
      },
      view_dimension,
    }),
    ImageClass::Sampled { kind, multi } => Ok(BindingType::Texture {
      sample_type: if multi {
        TextureSampleType::Depth
      } else {
        match kind {
          naga::ScalarKind::Float => TextureSampleType::Float { filterable: true },
          naga::ScalarKind::Sint => TextureSampleType::Sint,
          naga::ScalarKind::Uint => TextureSampleType::Uint,
          _ => TextureSampleType::Float { filterable: true },
        }
      },
      view_dimension,
      multisampled: multi,
    }),

    ImageClass::Depth { multi } => Ok(BindingType::Texture {
      sample_type: TextureSampleType::Depth,
      view_dimension,
      multisampled: multi,
    }),
    ImageClass::External => return Err(anyhow!("External images not allowed in shaders")),
  }
}
