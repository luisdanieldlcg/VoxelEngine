use std::vec;

use crate::{
    block::{Block, BlockId},
    world::chunk::{Chunk, CHUNK_X_SIZE, CHUNK_Y_SIZE, CHUNK_Z_SIZE, TOTAL_CHUNK_SIZE},
};
use vek::Vec3;

use super::{
    atlas::Atlas,
    buffer::compute_cube_indices,
    cube::CubePipeline,
    mesh::{vertex::Vertex, ChunkMesh},
    IRenderer,
};

pub const CHUNK_GRID_ROWS: usize = 2;
pub const CHUNK_GRID_COLS: usize = 2;
pub const CHUNK_GRID_SIZE: usize = CHUNK_GRID_ROWS * CHUNK_GRID_COLS;
pub const RENDER_DISTANCE: i32 = 1;

pub struct WorldRenderer {
    chunks: Vec<Chunk>,
    pipeline: CubePipeline,
    pipeline_wireframe: CubePipeline,
    pub atlas: Atlas,
    pub wireframe: bool,
    origin: Vec3<i32>,
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
        player_pos: Vec3<f32>,
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
        let chunks: Vec<Chunk> = vec![];

        let mut world = Self {
            chunks,
            pipeline: cube_pipeline,
            pipeline_wireframe: cube_wireframe_pipeline,
            atlas,
            wireframe: false,
            origin: Vec3::zero(),
        };
        world.load_initial_chunks(device, player_pos);
        world
    }

    pub fn load_initial_chunks(&mut self, device: &wgpu::Device, player_pos: Vec3<f32>) {
        let player_chunk_x = (player_pos.x / CHUNK_X_SIZE as f32).floor() as i32;
        let player_chunk_z = (player_pos.z / CHUNK_Z_SIZE as f32).floor() as i32;

        for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let chunk_x = player_chunk_x + (x * CHUNK_X_SIZE as i32);
                let chunk_z = player_chunk_z + (z * CHUNK_Z_SIZE as i32);

                let chunk_pos = Vec3::new(chunk_x, 0, chunk_z);
                self.chunks.push(Chunk::new(device, chunk_pos));
            }
        }
    }
    //        [] [] []             -32, 256, -64  example player pos
    //        [] [] []             -2,  256, -4   example chunk at player pos
    //        [] [] []
    //  []
    pub fn world_to_chunk_pos(&mut self, pos: Vec3<f32>) -> Vec3<i32> {
        let x = (pos.x as f32 / CHUNK_X_SIZE as f32).floor() as i32;
        let z = (pos.z as f32 / CHUNK_Z_SIZE as f32).floor() as i32;
        Vec3::new(x, 0, z)
    }

    pub fn load_chunks(&mut self, queue: &wgpu::Queue) {
        for (i, chunk) in self.chunks.iter_mut().enumerate() {
            let at = Vec3::new(
                i as i32 % CHUNK_GRID_ROWS as i32,
                0,
                i as i32 / CHUNK_GRID_SIZE as i32,
            );
            // Self::gen_chunk(&mut chunk.blocks, at);
            // chunk.mesh = Self::compute_mesh(chunk);
        }
        (0..CHUNK_GRID_SIZE).for_each(|i| {
            let chunk = &mut self.chunks[i];
            chunk.buffer.update(queue, &chunk.mesh);
        });
    }
    pub fn gen_chunk(blocks: &mut Vec<Vec<Vec<Block>>>, offset: Vec3<i32>) {
        (0..TOTAL_CHUNK_SIZE).into_iter().for_each(|i| {
            let z = i / (CHUNK_X_SIZE * CHUNK_Y_SIZE);
            let y = (i - z * CHUNK_X_SIZE * CHUNK_Y_SIZE) / CHUNK_X_SIZE;
            let x = i - CHUNK_X_SIZE * (y + CHUNK_Y_SIZE * z);
            // blocks[y][x][z].update(offset);
        });
    }

    // pub fn compute_mesh(chunk: &Chunk) -> ChunkMesh {
    //     let mut vertices: Vec<Vertex> = Vec::with_capacity(24 * TOTAL_CHUNK_SIZE);
    //     for y in 0..CHUNK_Y_SIZE {
    //         for z in 0..CHUNK_Z_SIZE {
    //             for x in 0..CHUNK_X_SIZE {
    //                 let block = &chunk.blocks[y][x][z];

    //                 let block_pos = block.pos();

    //                 for quad in &block.quads {
    //                     let normal = quad.dir.normalized();
    //                     let at = block_pos + normal;
    //                     if !Chunk::is_pos_in_bounds(at) {
    //                         vertices.extend_from_slice(&quad.vertices);
    //                         continue;
    //                     }
    //                     let neighbor_x = at.x as usize;
    //                     let neighbor_y = at.y as usize;
    //                     let neighbor_z = at.z as usize;

    //                     let neighbor = &chunk.blocks[neighbor_y][neighbor_x][neighbor_z];
    //                     if let BlockId::AIR = neighbor.id {
    //                         vertices.extend_from_slice(&quad.vertices);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     let indices = compute_cube_indices(vertices.len());
    //     ChunkMesh::new(vertices, indices)
    // }
    pub fn on_update(&mut self, player_pos: Vec3<f32>) {}
}
