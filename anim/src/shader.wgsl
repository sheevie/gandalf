@group(0) @binding(0)
var<uniform> theta: f32;

@vertex
fn vs_main(@location(0) position: vec3f) -> @builtin(position) vec4f{
    let s = sin(theta);
    let c = cos(theta);
    
    let rotated = vec3f(
        c * position.x - s * position.z,
        position.y,
        s * position.x + c * position.z,

    );
    let tilt = theta * 0.2;

    let ct = cos(tilt);
    let st = sin(tilt);

    let tilted = vec3f(
        rotated.x,
        ct * rotated.y - st * rotated.z,
        st * rotated.y + ct * rotated.z,
    );
    return vec4f(tilted.xy, tilted.z * 0.5 + 0.5, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 0.5, 0.0, 1.0);
}