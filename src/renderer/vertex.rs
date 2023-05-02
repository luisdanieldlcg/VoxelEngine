#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    vertex_pos: [f32; 3],
    texture_pos: [f32; 2],
}
impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub const POLYGON_VERTICES: &[Vertex] = &[
    Vertex {
        vertex_pos: [-0.0868241, 0.49240386, 0.0],
        texture_pos: [0.4131759, 0.99240386],
    }, // A - Red
    Vertex {
        vertex_pos: [-0.49513406, 0.06958647, 0.0],
        texture_pos: [0.0048659444, 0.56958647],
    }, // B - Orange
    Vertex {
        vertex_pos: [-0.21918549, -0.44939706, 0.0],
        texture_pos: [0.28081453, 0.05060294],
    }, // C - Yellow
    Vertex {
        vertex_pos: [0.35966998, -0.3473291, 0.0],
        texture_pos: [0.85967, 0.1526709],
    }, // D - Green
    Vertex {
        vertex_pos: [0.44147372, 0.2347359, 0.0],
        texture_pos: [0.9414737, 0.7347359],
    }, // E - Blue
];

pub const POLYGON_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
