use vek::Vec3;

use crate::chunk::Chunk;

use super::{atlas::Atlas, cube::CubePipeline, IRenderer};

pub struct WorldRenderer {
    chunk: Chunk,
    pipeline: CubePipeline,
    pipeline_wireframe: CubePipeline,
    pub atlas: Atlas,
    pub wireframe: bool,
}

impl IRenderer for WorldRenderer {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.wireframe {
            render_pass.set_pipeline(&self.pipeline_wireframe.pipeline);
        } else {
            render_pass.set_pipeline(&self.pipeline.pipeline);
        }
        render_pass.set_bind_group(0, &self.atlas.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.chunk.buffer.vertex_buf.buf.slice(..));
        render_pass.set_index_buffer(
            self.chunk.buffer.index_buf.buf.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..self.chunk.buffer.indices_len, 0, 0..1);
    }
}

impl WorldRenderer {
    pub fn new(
        sfc_cfg: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shader: wgpu::ShaderModule,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let texture_atlas = include_bytes!("../../assets/atlas.png");
        let atlas = Atlas::new(texture_atlas, &device, &queue);
        let cube_pipeline = CubePipeline::new(
            &device,
            &shader,
            &sfc_cfg,
            &[&atlas.bind_group_layout, &transform_bind_group_layout],
            wgpu::PolygonMode::Fill,
        );

        let cube_wireframe_pipeline = CubePipeline::new(
            &device,
            &shader,
            &sfc_cfg,
            &[&atlas.bind_group_layout, &transform_bind_group_layout],
            wgpu::PolygonMode::Line,
        );
        let chunk = Chunk::new(&device);
        let mut world = Self {
            chunk,
            pipeline: cube_pipeline,
            pipeline_wireframe: cube_wireframe_pipeline,
            atlas,
            wireframe: false,
        };
        Self::load_chunk(&mut world, queue);
        world
    }

    pub fn on_update(&mut self, player_pos: Vec3<f32>) {}

    fn load_chunk(&mut self, queue: &wgpu::Queue) {}
}
