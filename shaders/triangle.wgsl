struct VertexInput {
  @location(0) position: vec2f,
  @location(1) color: vec4f,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4f,
  @location(0) color: vec3f,
};

@vertex
fn vs_main(
  model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.color = model.color.xyz;
  out.clip_position = vec4f(model.position, 0.0, 1.0);
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
  return vec4f(in.color, 1.0);
}