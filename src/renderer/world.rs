use std::{vec, println};

use crate::{
    block::BlockId,
    world::chunk::{ Chunk, CHUNK_X_SIZE, CHUNK_Y_SIZE, CHUNK_Z_SIZE, TOTAL_CHUNK_SIZE},
};
use vek::Vec3;

use super::{
    atlas::Atlas,
    buffer::compute_cube_indices,
    cube::CubePipeline,
    mesh::{vertex::Vertex, ChunkMesh},
    IRenderer,
};

trait IWorldGenerator {
    fn initial_gen(&self, chunk_pos: Vec3<i32>) -> Chunk;
}
pub struct WorldRenderer {
    chunks: Vec<Chunk>,
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

        for chunk in &self.chunks {
            render_pass.set_vertex_buffer(0, chunk.buffer.vertex_buf.buf.slice(..));
            render_pass.set_index_buffer(
                chunk.buffer.index_buf.buf.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..chunk.buffer.indices_len, 0, 0..1);
        }
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
        let mut chunks = vec![];
        for x in 0..4 {
            chunks.push(Chunk::new(device, Vec3::new(x as i32 * 16, 1,  0)));
        }
        let mut world = Self {
            chunks,
            pipeline: cube_pipeline,
            pipeline_wireframe: cube_wireframe_pipeline,
            atlas,
            wireframe: false,
        };
        world.load_chunks(queue);

        world
    }

    pub fn load_chunks(&mut self, queue: &wgpu::Queue) {
        for chunk in &mut self.chunks {
            let new_mesh = Self::recreate_chunk(chunk);
            chunk.buffer.update(queue, &new_mesh);
        }
    }

    pub fn recreate_chunk(chunk: &mut Chunk) -> ChunkMesh {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(24 * TOTAL_CHUNK_SIZE);
        for y in 0..CHUNK_Y_SIZE {
            for z in 0..CHUNK_Z_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    let block = &chunk.blocks[y][x][z];
                    if let BlockId::AIR = block.id {
                        continue;
                    }
                    let block_pos = block.pos();
                    for quad in &block.quads {
                        let normal = quad.dir.normalized();
                        let at = block_pos + normal;
                        if !Chunk::is_pos_in_bounds(at) {
                            vertices.extend_from_slice(&quad.vertices);
                            continue;
                        }
                        let neighbor = &chunk.blocks[at.y as usize][at.x as usize][at.z as usize];
                        if let BlockId::AIR = neighbor.id {
                            vertices.extend_from_slice(&quad.vertices);
                        }
                    }
                }
            }
        }
        let indices = compute_cube_indices(vertices.len());
        ChunkMesh::new(vertices, indices)
    }
    pub fn on_update(&mut self, player_pos: Vec3<f32>) {
        
    }
}
