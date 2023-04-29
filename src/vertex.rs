#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color attribute
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
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
