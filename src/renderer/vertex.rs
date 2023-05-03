#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
}
impl Vertex {
    pub fn new(p1: f32, p2: f32, p3: f32) -> Self {
        Self { pos: [p1, p2, p3] }
    }
}
pub trait IVertex: Clone + bytemuck::Pod {
    const STRIDE: wgpu::BufferAddress;
    const INDEX_FORMAT: Option<wgpu::IndexFormat>;
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
    const INDEX_FORMAT: Option<wgpu::IndexFormat> = None;
}

pub const POLYGON_VERTICES: &[Vertex] = &[
    Vertex {
        pos: [-0.0868241, 0.49240386, 0.0],
        // texture_pos: [0.4131759, 0.99240386],
    }, // A - Red
    Vertex {
        pos: [-0.49513406, 0.06958647, 0.0],
        // texture_pos: [0.0048659444, 0.56958647],
    }, // B - Orange
    Vertex {
        pos: [-0.21918549, -0.44939706, 0.0],
        //texture_pos: [0.28081453, 0.05060294],
    }, // C - Yellow
    Vertex {
        pos: [0.35966998, -0.3473291, 0.0],
        // texture_pos: [0.85967, 0.1526709],
    }, // D - Green
    Vertex {
        pos: [0.44147372, 0.2347359, 0.0],
        // texture_pos: [0.9414737, 0.7347359],
    }, // E - Blue
];

pub const POLYGON_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
