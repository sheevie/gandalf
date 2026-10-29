@group(0) @binding(0)
var<uniform> theta: f32;

@vertex
fn vs_main(@location(0) position: vec3f) -> @builtin(position) vec4f{
    let s = sin(theta);
    let c = cos(theta);
    
    let rotated = vec3f(
        position.x + c * 0.1,
        c * position.y + s * position.z + s * 0.1,
        -s * position.y + c * position.z

    );
    return vec4f(rotated.xy, rotated.z * 0.5 + 0.5, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 0.5, 0.0, 1.0);
}