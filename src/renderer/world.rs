use std::{println, vec};

use crate::{
    block::Block,
    world::chunk::{Chunk, CHUNK_X_SIZE, CHUNK_Y_SIZE, CHUNK_Z_SIZE, TOTAL_CHUNK_SIZE}, scene::camera::Camera,
};
use vek::Vec3;

use super::{atlas::Atlas, cube::CubePipeline, IRenderer};

pub const CHUNK_GRID_ROWS: usize = 2;
pub const CHUNK_GRID_COLS: usize = 2;
pub const CHUNK_GRID_SIZE: usize = CHUNK_GRID_ROWS * CHUNK_GRID_COLS;
pub const RENDER_DISTANCE: i32 = 2;

pub struct WorldRenderer {
    chunks: Vec<Chunk>,
    pipeline: CubePipeline,
    pipeline_wireframe: CubePipeline,
    pub atlas: Atlas,
    pub wireframe: bool,
    center_offset: Vec3<i32>,
    chunks_origin: Vec3<i32>,
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
        camera: &mut Camera
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

        let center_offset = Vec3::new(0, 0, 0);
        let chunks_origin = Vec3::new(RENDER_DISTANCE as i32 / 2, 0, RENDER_DISTANCE as i32 / 2);

        let mut world = Self {
            chunks,
            pipeline: cube_pipeline,
            pipeline_wireframe: cube_wireframe_pipeline,
            atlas,
            wireframe: false,
            center_offset,
            chunks_origin,
        };
        world.load_initial_chunks(device, camera);
        world
    }

    pub fn set_center_at(&mut self, block_pos: Vec3<f32>) {
        let chunk_pos = self.world_to_chunk_pos(block_pos);
        let chunks_origin = chunk_pos - Vec3::new(RENDER_DISTANCE / 2, 0, RENDER_DISTANCE / 2);
        println!("Chunk pos: {:?}", chunk_pos);
        println!("Chunks origin: {:?}", chunks_origin);

        if chunks_origin == self.chunks_origin {
            println!("Chunks origin is the same, no need to update");
            return;
        }
        // Update the center
        self.center_offset = chunk_pos;
        self.chunks_origin = chunks_origin;

    }
    pub fn on_update(&mut self, player_pos: Vec3<f32>, device: &wgpu::Device) {
       self.set_center_at(player_pos);
    }

    pub fn load_initial_chunks(&mut self, device: &wgpu::Device, camera: &mut Camera) {
        let player_chunk_pos: Vec3<i32> = self.world_to_chunk_pos(camera.pos);

        for x in -RENDER_DISTANCE..=RENDER_DISTANCE  {
            for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let chunk_pos = player_chunk_pos + Vec3::new(x, 0, z);
                let world_pos = self.chunk_pos_to_world_pos(chunk_pos);
                self.chunks.push(Chunk::new(device, world_pos));
            }
        }
    }
    
    /// Returns the chunk at the given world position
    pub fn world_to_chunk_pos(&mut self, pos: Vec3<f32>) -> Vec3<i32> {
        let x = (pos.x  / CHUNK_X_SIZE as f32).floor() as i32;
        let y = (pos.y  / CHUNK_Y_SIZE as f32).floor() as i32;
        let z = (pos.z  / CHUNK_Z_SIZE as f32).floor() as i32;
        Vec3::new(x, y, z)
    }
    /// Returns the world position of the given chunk
    pub fn chunk_pos_to_world_pos(&mut self, chunk_pos: Vec3<i32>) -> Vec3<i32> {
        let x = chunk_pos.x * CHUNK_X_SIZE as i32;
        let z = chunk_pos.z * CHUNK_Z_SIZE as i32;
        Vec3::new(x, chunk_pos.y, z)
    }



}
