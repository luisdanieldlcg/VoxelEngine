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

pub struct LineMesh {
    vertices: Vec<LineVertex>,
}

impl LineMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn line(offset: Vec3<i32>) -> Self {
        let mut mesh = LineMesh::new();
        mesh.add_line([[0, 0, 0], [1, 0, 0]]);
        mesh
    }

    pub fn make_cube_mesh() -> (Vec<LineVertex>, Vec<u16>) {
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
            LineVertex::new([0, 0, 0], c),
            LineVertex::new([UNIT, 0, 0], c),
            LineVertex::new([UNIT, UNIT, 0], c),
            LineVertex::new([0, UNIT, 0], c),
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
    pub fn cube(offset: Vec3<i32>) -> Self {
        let mut mesh = LineMesh::new();
        // Facing +Z
        mesh.add_line([[0, 0, 0], [1, 0, 0]]);
        mesh.add_line([[1, 0, 0], [1, 0, 1]]);
        mesh.add_line([[1, 0, 1], [0, 0, 1]]);
        mesh.add_line([[0, 0, 1], [0, 0, 0]]);
        mesh.add_line([[0, 1, 0], [1, 1, 0]]);
        mesh.add_line([[1, 1, 0], [1, 1, 1]]);
        mesh.add_line([[1, 1, 1], [0, 1, 1]]);
        mesh.add_line([[0, 1, 1], [0, 1, 0]]);
        mesh.add_line([[0, 0, 0], [0, 1, 0]]);
        mesh.add_line([[1, 0, 0], [1, 1, 0]]);
        mesh.add_line([[1, 0, 1], [1, 1, 1]]);
        mesh.add_line([[0, 0, 1], [0, 1, 1]]);
        mesh
    }

    pub fn add_line(&mut self, line: [[i32; 3]; 2]) {
        let from = line[0];
        let to = line[1];
        self.vertices.push(LineVertex::new(from, [1.0, 0.0, 0.0]));
        self.vertices.push(LineVertex::new(to, [1.0, 0.0, 0.0]));
    }

    pub fn compute_indices(&self) -> Vec<u16> {
        let len = self.vertices.len();
        let mut indices = Vec::with_capacity(len as usize);
        // single line indices
        if len == 2 {
            return vec![0, 1];
        }
        // example cube indices
        // the index increment by 1 each 3 vertices
        // and the forth index is the current index - 3
        // compute cube face indices
        for i in (0..len).step_by(3) {
            let index = i as u16;
            indices.push(index);
            indices.push(index + 1);
            indices.push(index + 1);
            indices.push(index + 2);
            indices.push(index + 2);
            indices.push(index + 3);
            indices.push(index);
        }
        indices
    }

    pub fn vertices(&self) -> &[LineVertex] {
        &self.vertices
    }
}


pub fn make_coordinate_mesh() -> (Vec<LineVertex>, Vec<u16>) {
    let color = [1.0, 0.1, 0.0];
    const UNIT: i32 = 1;
    let mut data = [
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
