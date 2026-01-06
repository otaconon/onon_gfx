use std::{rc::Rc};

use wgpu::{
  BindGroupLayout, Device, PipelineLayout
};

use crate::render_resource::{Shader, shader::ShaderBindingInfo};

pub struct ShaderEffect {
  shader: Rc<Shader>,
  pipeline_layout: PipelineLayout,
  bind_group_layouts: Vec<BindGroupLayout>,
}

impl ShaderEffect {
  pub fn new(device: &Device, shader: Rc<Shader>) -> Self {
    // Bindings are sorted by group first, binding second
    let last_group = shader.bindings.last();
    let Some(last_group) = last_group else {
      return Self {
        shader,
        pipeline_layout: device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
          label: Some("Render Pipeline Layout"),
          bind_group_layouts: &[],
          immediate_size: 12,
        }),
        bind_group_layouts: Vec::new(),
      };
    };

    let groups_count = last_group.group as usize + 1;
    let mut groups: Vec<Vec<&ShaderBindingInfo>> = vec![Vec::new(); groups_count];

    for binding in &shader.bindings {
      groups[binding.group as usize].push(&binding);
    }

    let bind_group_layouts: Vec<BindGroupLayout> = groups
      .into_iter()
      .map(|group_binding_infos| create_layout_from_bindings(device, &group_binding_infos))
      .collect();

    let bind_group_layout_refs: Vec<&wgpu::BindGroupLayout> = bind_group_layouts.iter().collect();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &bind_group_layout_refs,
      immediate_size: 12,
    });

    Self {
      shader,
      pipeline_layout,
      bind_group_layouts,
    }
  }
}

fn create_layout_from_bindings(
  device: &Device,
  binding_infos: &Vec<&ShaderBindingInfo>,
) -> BindGroupLayout {
  let entries: Vec<wgpu::BindGroupLayoutEntry> = binding_infos
    .iter()
    .map(|infos| wgpu::BindGroupLayoutEntry {
      binding: infos.binding,
      visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
      ty: infos.ty,
      count: None,
    })
    .collect();

  device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: None,
    entries: &entries,
  })
}
