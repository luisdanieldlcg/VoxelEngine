use super::{buffer::Buffer, mesh::line::{LineVertex, create_vertices}, pipelines::debug::DebugPipeline, Renderable};


impl Renderable for DebugRenderer {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, global_uniforms: &'a wgpu::BindGroup) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        render_pass.set_bind_group(0, global_uniforms, &[]);
        render_pass.set_vertex_buffer(0, self.buffer.buf.slice(..));
        render_pass.set_index_buffer(self.indices.buf.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices as u32, 0, 0..1);
    }
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
