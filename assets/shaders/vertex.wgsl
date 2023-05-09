// pos is a reserved keyword in Metal.

struct Camera {
    transform: mat4x4<f32>,
}

@group(1) @binding(0) 
var<uniform> camera: Camera;

struct CubeInstance {
   @location(2) translation: vec3<f32>,
}

struct VertexIn {
    @location(0) vertex_pos: vec3<f32>,
    @location(1) texture_pos: vec2<f32>,
}

struct VertexData {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) texture_pos: vec2<f32>,
}

@vertex
fn vs_main(in: VertexIn, instance: CubeInstance) -> VertexData {
    var data: VertexData;
    data.texture_pos = in.texture_pos;

    var cube_instance: CubeInstance;
    cube_instance.translation = instance.translation;
    let pos = in.vertex_pos + cube_instance.translation;
    data.vertex_pos = camera.transform * vec4<f32>(pos, 1.0);
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