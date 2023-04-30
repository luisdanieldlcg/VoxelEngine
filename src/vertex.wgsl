struct Camera {
    transform: mat4x4<f32>,
}

@group(1) @binding(0) 
var<uniform> camera: Camera;

struct VertexIn {
    // Vertices coordinates
    @location(0) vertices: vec3<f32>,
    // Texture coordinates
    @location(1) tex_pos: vec2<f32>,
}

struct VertexData {
    @builtin(position) vertices: vec4<f32>,
    @location(0) tex_pos: vec2<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexData {
    var data: VertexData;
    data.vertices = camera.transform * vec4<f32>(in.vertices, 1.0);
    data.tex_pos = in.tex_pos;
    return data;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(data: VertexData) -> @location(0) vec4<f32> {
    return textureSample(texture, tex_sampler, data.tex_pos);
}