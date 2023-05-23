struct VertexIn {
    @location(0) vertex_pos: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOut {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

struct Camera {
    transform: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(input: VertexIn) -> VertexOut {
    var data: VertexOut;
    data.vertex_pos = camera.transform * vec4<f32>(input.vertex_pos, 1.0);
    data.color = input.color;
    return data;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}