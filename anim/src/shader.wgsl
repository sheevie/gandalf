struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 4>(
        vec2(0.0, 1.0),
        vec2(1.0, 0.0),
        vec2(0.0, -1.0),
        vec2(-1.0, 0.0)
    );

    var color = array<vec4f, 3>(
        vec4f(1, 0, 0, 1),
        vec4f(0, 1, 0, 1),
        vec4f(0, 0, 1, 1),
    );

    var result: VertexOutput;

    result.position = vec4f(positions[idx], 0.0, 1.0);
    result.color = color[idx];

    return result;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let a = vec4f(0.5,0.1,0,1);
    let b = vec4f(1,1,1,1);
    // let grid = vec2u(in.position.xy) / 8;
    // let checker = (grid.x + grid.y) % 2 == 1;
    return a;
    // return vec4(1.0, 0.5, 0.0, 1.0);
}