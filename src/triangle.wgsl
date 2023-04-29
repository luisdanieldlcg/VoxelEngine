struct VertexIn {
    @location(0) vertices: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexData {
    @builtin(position) vertices: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexData {
    var data: VertexData;
    data.vertices = vec4<f32>(in.vertices, 1.0);
    data.color = vec4<f32>(in.color, 1.0);
    return data;
}

@fragment
fn fs_main(data: VertexData) -> @location(0) vec4<f32> {
    return data.color;
}