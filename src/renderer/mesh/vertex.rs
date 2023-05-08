use crate::renderer::atlas::atlas_uv_mapping;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}

impl Vertex {
    pub fn new(v1: i8, v2: i8, v3: i8, uv: [u8; 2], texture_id: u8) -> Self {
        Self {
            pos: [v1 as f32, v2 as f32, v3 as f32],
            uv: atlas_uv_mapping(texture_id, uv[0], uv[1]),
        }
    }
}

impl Vertex {
    pub const INDEX_BUFFER_FORMAT: Option<wgpu::IndexFormat> = Some(wgpu::IndexFormat::Uint16);

    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
