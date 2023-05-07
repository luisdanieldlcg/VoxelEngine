struct Camera {
    transform: mat4x4<f32>,
}

@group(1) @binding(0) 
var<uniform> camera: Camera;

struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) texture_pos: vec2<f32>,
}

struct VertexData {
    @builtin(position) pos: vec4<f32>,
    @location(0) texture_pos: vec2<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexData {
    var data: VertexData;
    data.pos = camera.transform * vec4<f32>(in.pos, 1.0);
    data.texture_pos = in.texture_pos;
    return data;
} 

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(data: VertexData) -> @location(0) vec4<f32> {
    return textureSample(texture, tex_sampler, atlas_uv_mapping(0.0, data.pos.x, data.pos.y));
}

// This function will map the vertices of a quad to texture coordinates
// A few things to consider:
// First thing is that the block atlas has a size of 256px and is composed of
// 16x16 textures.
// In WGPU the origin is located at the top left corner and the y axis is inverted
// so we need to invert the y axis. Also, the range belongs to [0, 1].
// Given a texture id, x and y coordinates, this function will return the
// texture coordinates of the quad.
// TODO: make this work
fn atlas_uv_mapping(texture_id: f32, x: f32, y: f32) -> vec2<f32> {
    let atlas_size = 256.0;
    let texture_size = 1.0 / 256.0;
    let u = 0.0;
    let v = 0.0;
    return vec2<f32>(u, v);
}