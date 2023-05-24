use std::vec;

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

    pub fn new(at: [i32; 3], color: [f32; 3]) -> Self {
        Self {
            pos: [at[0] as f32, at[1] as f32 + 257.0, at[2] as f32],
            color,
        }
    }
}

pub fn make_coordinate_mesh() -> (Vec<LineVertex>, Vec<u16>) {
    let color = [1.0, 0.1, 0.0];
    const UNIT: i32 = 1;
    let data = [
        // X
        LineVertex::new([0, 0, 0], color),
        LineVertex::new([UNIT, 0, 0], color),
        // Y
        LineVertex::new([0, 0, 0], color),
        LineVertex::new([0, UNIT, 0], color),
        // Z
        LineVertex::new([0, 0, 0], color),
        LineVertex::new([0, 0, UNIT], color),
    ];
    // data.iter_mut().for_each(|v| v.pos[1] += 256.0);
    let index_data: &[u16] = &[
        0, 1, //z
        2, 3, //x
        4, 5, //y
    ];

    (data.to_vec(), index_data.to_vec())
}

pub fn make_line_mesh(world_pos: Vec3<i32>) -> ([LineVertex; 2], [u16; 2]) {
    let from = [world_pos.x, world_pos.y, world_pos.z];
    let to = [1 + world_pos.x, world_pos.y, world_pos.z];
    
    let indices = [0, 1];
    let vertices = [
        LineVertex::new(from, [1.0, 0.0, 0.0]),
        LineVertex::new(to, [1.0, 0.0, 0.0]),
    ];
    (vertices, indices)
}
pub fn make_cube_mesh(offset: Vec3<i32>) -> (Vec<LineVertex>, Vec<u16>) {
    let c = [1.0, 0.0, 0.0];
    const UNIT: i32 = 1;

    let indices = vec![
        // +Z
        0, 1, 1, 2, 2, 3, 3, 0, // -Z
        4, 5, 5, 6, 6, 7, 7, 4, // +X
        8, 9, 9, 10, 10, 11, 11, 8, // -X
        12, 13, 13, 14, 14, 15, 15, 12, // +Y
        16, 17, 17, 18, 18, 19, 19, 16, // -Y
        20, 21, 21, 22, 22, 23, 23, 20,
    ];

    let vertices: Vec<LineVertex> = vec![
        // Facing +Z

        // Facing -Z
        LineVertex::new([0, 0, UNIT], c),
        LineVertex::new([UNIT, 0, UNIT], c),
        LineVertex::new([UNIT, UNIT, UNIT], c),
        LineVertex::new([0, UNIT, UNIT], c),
        // Facing +X
        LineVertex::new([UNIT, 0, 0], c),
        LineVertex::new([UNIT, 0, UNIT], c),
        LineVertex::new([UNIT, UNIT, UNIT], c),
        LineVertex::new([UNIT, UNIT, 0], c),
        // Facing -X
        LineVertex::new([0, 0, 0], c),
        LineVertex::new([0, 0, UNIT], c),
        LineVertex::new([0, UNIT, UNIT], c),
        LineVertex::new([0, UNIT, 0], c),
        // Facing +Y
        LineVertex::new([0, UNIT, 0], c),
        LineVertex::new([UNIT, UNIT, 0], c),
        LineVertex::new([UNIT, UNIT, UNIT], c),
        LineVertex::new([0, UNIT, UNIT], c),
        // Facing -Y
        LineVertex::new([0, 0, 0], c),
        LineVertex::new([UNIT, 0, 0], c),
        LineVertex::new([UNIT, 0, UNIT], c),
        LineVertex::new([0, 0, UNIT], c),
    ];

    (vertices, indices)
}
