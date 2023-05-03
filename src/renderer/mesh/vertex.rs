#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
}
pub trait IVertex: Clone + bytemuck::Pod {
    const STRIDE: wgpu::BufferAddress;
    const INDEX_BUFFER_FORMAT: Option<wgpu::IndexFormat>;
}

impl Vertex {
    pub fn new(p1: f32, p2: f32, p3: f32) -> Self {
        Self { pos: [p1, p2, p3] }
    }
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        // TODO: remove vertex attribute 1
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: Self::STRIDE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl IVertex for Vertex {
    const STRIDE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    const INDEX_BUFFER_FORMAT: Option<wgpu::IndexFormat> = Some(wgpu::IndexFormat::Uint16);
}
