use vek::Vec3;

use super::{
    buffer::Buffer,
    mesh::line::{LineVertex, make_cube_mesh, make_line_mesh},
    pipelines::debug::DebugPipeline,
    Renderable,
};

impl Renderable for DebugRenderer {
    fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        global_uniforms: &'a wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        render_pass.set_bind_group(0, global_uniforms, &[]);
        self.line.render(render_pass, global_uniforms);
        self.cube.render(render_pass, global_uniforms);
    }
}

pub struct DebugRenderer {
    line: LineRenderer,
    cube: LineRenderer,
    pipeline: DebugPipeline,
}

impl DebugRenderer {
    pub fn new(
        device: &wgpu::Device,
        sfc: &wgpu::SurfaceConfiguration,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let cube = make_cube_mesh(Vec3::zero());
        let line = make_line_mesh(Vec3::new(0, 2, 2));
        let pipeline = DebugPipeline::new(device, &sfc, &[transform_bind_group_layout]);

        Self {
            line: LineRenderer::new(device, &line.0, &line.1),
            cube: LineRenderer::new(device, &cube.0, &cube.1),
            pipeline,
        }
    }
}
pub struct LineRenderer {
    buffer: Buffer<LineVertex>,
    indices: Buffer<u16>,
    num_indices: u32,
}

impl Renderable for LineRenderer {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, _: &'a wgpu::BindGroup) {
        render_pass.set_index_buffer(self.indices.buf.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, self.buffer.buf.slice(..));
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
impl LineRenderer {
    pub fn new(device: &wgpu::Device, vertices: &[LineVertex], indices: &[u16]) -> Self {
        let line_vertex_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            vertices,
        );
        let line_index_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            indices,
        );
        Self {
            buffer: line_vertex_buffer,
            indices: line_index_buffer,
            num_indices: indices.len() as u32,
        }
    }
}
