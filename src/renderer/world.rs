use crate::{
    block::{Block, BlockId},
    chunk::Chunk,
};

use super::{
    atlas::Atlas,
    buffer::{create_cube_index_buffer, Buffer},
    cube::CubePipeline,
    mesh::{vertex::Vertex, Mesh},
    IRenderer,
};

pub struct WorldRenderer {
    chunk: Chunk,
    pipeline: CubePipeline,
    pipeline_wireframe: CubePipeline,
    cube_buffer: Buffer<Vertex>,
    cube_index_buffer: Buffer<u16>,
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
        render_pass.set_vertex_buffer(0, self.chunk.buffer.vertex().buf.slice(..));
        render_pass.set_index_buffer(
            self.chunk.buffer.index().buf.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.chunk.buffer.index().len() as u32, 0, 0..1 as u32);
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
        let dirt = Block::new(BlockId::DIRT, [0.0, 0.0, 0.0]);
        let cube = Mesh::cube(dirt.id());

        let cube_buffer = Buffer::new(&device, wgpu::BufferUsages::VERTEX, &cube.vertices());
        let cube_index_buffer = create_cube_index_buffer(&device);
      
        Self {
            chunk,
            pipeline: cube_pipeline,
            pipeline_wireframe: cube_wireframe_pipeline,
            atlas,
            wireframe: false,
            cube_index_buffer,
            cube_buffer,
        }
    }
}
