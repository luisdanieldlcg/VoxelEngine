use vek::Vec3;

use super::{buffer::Buffer, pipelines::debug::DebugPipeline, Renderable};

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

    pub fn draw_line(at: Vec3<i32>, color: [f32; 3]) -> Self {
        Self {
            pos: [at.x as f32, at.y as f32, at.z as f32],
            color,
        }
    }
}

impl Renderable for DebugRenderer {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        render_pass.set_vertex_buffer(0, self.buffer.buf.slice(..));
        render_pass.set_index_buffer(self.indices.buf.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices as u32, 0, 0..1);
    }
}

fn coordinate_vertex(pos: [i8; 3], c: [u8; 3]) -> LineVertex {
    LineVertex {
        pos: [pos[0] as f32, pos[1] as f32 + 256.0, pos[2] as f32],
        color: [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0],
    }
}
pub fn create_vertices() -> (Vec<LineVertex>, Vec<u16>) {
    let vertex_data = [
        // z
        coordinate_vertex([0, 0, 0], [0, 0, 255]),
        coordinate_vertex([0, 0, 2], [0, 0, 255]),
        // x
        coordinate_vertex([0, 0, 0], [255, 0, 0]),
        coordinate_vertex([2, 0, 0], [255, 0, 0]),
        // y
        coordinate_vertex([0, 0, 0], [0, 255, 0]),
        coordinate_vertex([0, 2, 0], [0, 255, 0]),
    ];

    let index_data: &[u16] = &[
        0, 1, //z
        2, 3, //x
        4, 5, //y
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}
pub struct DebugRenderer {
    buffer: Buffer<LineVertex>,
    indices: Buffer<u16>,
    pipeline: DebugPipeline,
    num_indices: usize,
}

impl DebugRenderer {
    pub fn new(
        device: &wgpu::Device,
        sfc: &wgpu::SurfaceConfiguration,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let data = create_vertices();
        let pipeline = DebugPipeline::new(device, &sfc, &[transform_bind_group_layout]);

        let buffer = Buffer::new(
            device,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            &data.0,
        );
        let indices = Buffer::new(
            device,
            wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            &data.1,
        );
        Self { buffer, pipeline, indices, num_indices: data.1.len() }
    }
}
