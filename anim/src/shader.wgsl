@vertex
fn vs_main(
    @location(0) position: vec2f, @builtin(vertex_index) idx: u32
) -> @builtin(position) vec4f{
    var color = array<vec4f, 3>(
        vec4f(1, 0, 0, 1),
        vec4f(0, 1, 0, 1),
        vec4f(0, 0, 1, 1),
    );

    return vec4f(position, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 0.5, 0.0, 1.0);
}