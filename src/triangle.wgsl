struct VertexData {
   @builtin(position) vertices: vec4<f32>,
}

@vertex
fn vs_main() -> VertexData{
    var data: VertexData;
    return data;
}

@fragment
fn fs_main(input: VertexData) -> vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}