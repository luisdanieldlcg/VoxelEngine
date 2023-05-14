use crate::{
    block::BlockId,
    world::chunk::{Chunk, CHUNK_X_SIZE, CHUNK_Y_SIZE, CHUNK_Z_SIZE, TOTAL_CHUNK_SIZE},
};
use vek::Vec3;

use super::{atlas::Atlas, cube::CubePipeline, IRenderer, mesh::{vertex::Vertex, ChunkMesh}, buffer::compute_cube_indices};

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

        let mut world = Self {
            chunks: vec![Chunk::new(device, Vec3::zero())],
            pipeline: cube_pipeline,
            pipeline_wireframe: cube_wireframe_pipeline,
            atlas,
            wireframe: false,
        };
        world.load_chunk(queue, Vec3::zero());

        
        world
    }

    pub fn load_chunk(&mut self, queue: &wgpu::Queue, chunk_pos: Vec3<i32>) {
        let new_mesh = self.recreate_chunk();
        self.chunks[0].buffer.update(queue, &new_mesh);
    }

    pub fn recreate_chunk(&mut self) -> ChunkMesh{
        let mut vertices: Vec<Vertex> = Vec::with_capacity(24 * TOTAL_CHUNK_SIZE);
        for y in 0..CHUNK_Y_SIZE {
            for z in 0..CHUNK_Z_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    let block = &self.chunks[0].blocks[y][x][z];
                    if let BlockId::AIR = block.id {
                        continue;
                    }
                    let block_pos = block.pos();
                    for quad in &block.quads {
                        let normal = quad.dir.normalized();
                        let neighbor_pos = block_pos + normal;
                        let mut visible = false;
                        if Chunk::is_pos_in_bounds(neighbor_pos) {
                            let neighbor_block = &self.chunks[0].blocks[neighbor_pos.y as usize]
                                [neighbor_pos.x as usize][neighbor_pos.z as usize];
                            
                            if let BlockId::AIR = neighbor_block.id {
                                visible = true;
                            }
                        } else {
                            visible = true;
                        }
                        if visible {
                            vertices.extend_from_slice(&quad.vertices);
                        }
                       
                    }
                    
                }
            }
        }
        let indices = compute_cube_indices(vertices.len());
        ChunkMesh {
            num_elements: indices.len() as u32,
            vertices,
            indices,
        }
    }
    pub fn on_update(&mut self, player_pos: Vec3<f32>) {}
    fn update_chunks(&mut self) {}
}
