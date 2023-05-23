use egui::plot::Line;
use vek::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct LineVertex {
    pos: [f32; 3],
    color: [f32; 3],
}
impl LineVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    pub fn new(at: Vec3<i32>, color: [f32; 3]) -> Self {
        Self {
            pos: [at.x as f32, at.y as f32, at.z as f32],
            color,
        }
    }
}

pub fn create_lines() -> (Vec<LineVertex>, Vec<u16>) {
    let color = [1.0, 0.1, 0.1];
    const UNIT: i32 = 1;
    let mut data = [
        // X
        LineVertex::new(Vec3::new(0, 0, 0), color),
        LineVertex::new(Vec3::new(UNIT, 0, 0), color),
        // Y
        LineVertex::new(Vec3::new(0, 0, 0), color),
        LineVertex::new(Vec3::new(0, UNIT, 0), color),
        // Z
        LineVertex::new(Vec3::new(0, 0, 0), color),
        LineVertex::new(Vec3::new(0, 0, UNIT), color),
    ];
    data.iter_mut().for_each(|v| v.pos[1] += 256.0);
    let index_data: &[u16] = &[
        0, 1, //z
        2, 3, //x
        4, 5, //y
    ];

    (data.to_vec(), index_data.to_vec())
}
