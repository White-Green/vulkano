@binding(0) @group(0) var<storage> frame : array<vec4f>;
@vertex
fn vert_main(@builtin(vertex_index) vertex_index : u32) -> @builtin(position) vec4f {
  return frame[vertex_index];
}
