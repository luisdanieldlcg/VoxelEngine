
struct VertexIn {
    @location(0) vertex_pos: vec3<f32>,
    @location(1) texture_pos: vec2<f32>,
}

struct VertexData {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) texture_pos: vec2<f32>,
}

struct Camera {
    transform: mat4x4<f32>,
}

@group(1) @binding(0) 
var<uniform> camera: Camera;

@vertex
fn vs_main(in: VertexIn) -> VertexData {
    var data: VertexData;
    data.vertex_pos = camera.transform * vec4<f32>(in.vertex_pos, 1.0);
    data.texture_pos = in.texture_pos;
    return data;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(data: VertexData) -> @location(0) vec4<f32> {
    return textureSample(texture, tex_sampler, data.texture_pos);
}