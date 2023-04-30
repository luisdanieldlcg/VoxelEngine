#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}
impl Vertex {
     
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
pub const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex {
        pos: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub const POLYGON_VERTICES: &[Vertex] = &[
    Vertex {
        pos: [-0.0868241, 0.49240386, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // A - Red
    Vertex {
        pos: [-0.49513406, 0.06958647, 0.0],
        color: [1.0, 0.5, 0.0],
    }, // B - Orange
    Vertex {
        pos: [-0.21918549, -0.44939706, 0.0],
        color: [1.0, 1.0, 0.0],
    }, // C - Yellow
    Vertex {
        pos: [0.35966998, -0.3473291, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // D - Green
    Vertex {
        pos: [0.44147372, 0.2347359, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // E - Blue
];

pub const POLYGON_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
// 0, 1, 4,
// 1, 2, 4,
// 2, 3, 4,
