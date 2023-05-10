use crate::{
    block::{Block, BlockId},
    chunk::Chunk,
};

use super::{
    atlas::Atlas,
    buffer::{create_quad_index_buffer, Buffer},
    cube::CubePipeline,
    instance::Instance,
    mesh::{vertex::Vertex, Mesh},
    IRenderer,
};

pub struct WorldRenderer {
    chunk: Chunk,
    pipeline: CubePipeline,
    pipeline_wireframe: CubePipeline,
    quad_buffer: Buffer<Vertex>,
    quad_index_buffer: Buffer<u16>,
    instance_buffer: Buffer<Instance>,
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
        render_pass.set_vertex_buffer(0, self.quad_buffer.buf.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.buf.slice(..));
        render_pass.set_index_buffer(
            self.quad_index_buffer.buf.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(
            0..self.quad_index_buffer.len() as u32,
            0,
            0..self.instance_buffer.len() as u32,
        );
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
        let chunk = Chunk::new();

        let mut instances = Vec::new();
        let block_size = 2.0;
        let plane_size = 100.0;
        for x in 0..plane_size as i32 {
            for z in 0..plane_size as i32 {
                instances.push(Instance {
                    transform: [x as f32 * block_size, 0.0, z as f32 * block_size],
                });
            }
        }
        let dirt = Block::new(BlockId::DIRT, [0.0, 0.0, 0.0]);
        let cube = Mesh::cube(dirt.id());

        let quad_buffer = Buffer::new(&device, wgpu::BufferUsages::VERTEX, &cube.vertices());
        let quad_index_buffer = create_quad_index_buffer(&device);
        let instance_buffer: Buffer<Instance> =
            Buffer::new(&device, wgpu::BufferUsages::VERTEX, &instances);

        Self {
            chunk,
            pipeline: cube_pipeline,
            pipeline_wireframe: cube_wireframe_pipeline,
            atlas,
            wireframe: false,
            quad_index_buffer,
            quad_buffer,
            instance_buffer,
        }
    }
}
