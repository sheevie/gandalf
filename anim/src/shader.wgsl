struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(@location(0) position: vec2f, @builtin(vertex_index) idx: u32) -> VertexOutput {
    var color = array<vec4f, 3>(
        vec4f(1, 0, 0, 1),
        vec4f(0, 1, 0, 1),
        vec4f(0, 0, 1, 1),
    );

    var result: VertexOutput;

    result.position = vec4f(position, 0.0, 1.0);
    result.color = select(color[0], color[1], idx > 2);

    return result;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let a = vec4f(0.5,0.1,0,1);
    let b = vec4f(0.5,1,1,1);
    // let grid = vec2u(in.position.xy) / 8;
    // let checker = (grid.x + grid.y) % 2 == 1;
    return in.color;
    // return vec4(1.0, 0.5, 0.0, 1.0);
}