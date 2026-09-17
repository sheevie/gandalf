@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2(0.0, 0.5),
        vec2(-0.25, -0.5),
        vec2(0.5, -0.5),
    );
    return vec4(positions[idx], 0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 0.5, 0.0, 1.0);
}