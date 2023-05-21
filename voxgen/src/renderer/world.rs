use std::collections::HashSet;

use log::info;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use vek::Vec3;

use crate::{
    scene::camera::Camera,
    world::chunk::{Chunk, ChunkPos},
};

use super::{atlas::Atlas, pipelines::voxel::VoxelPipeline, Renderable};

pub const RENDER_DISTANCE: i32 = 4;

pub struct WorldRenderer {
    pub chunks: Vec<Chunk>,
    pub chunks_pos: HashSet<ChunkPos>,
    pipeline: VoxelPipeline,
    pipeline_wireframe: VoxelPipeline,
    pub wireframe: bool,
    pub atlas: Atlas,
}

impl Renderable for WorldRenderer {
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
        camera: &Camera,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shader: &wgpu::ShaderModule,
        cfg: &wgpu::SurfaceConfiguration,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let texture_atlas = include_bytes!("../../../assets/atlas.png");
        let atlas = Atlas::new(texture_atlas, &device, &queue);
        let pipeline = VoxelPipeline::new(
            device,
            shader,
            cfg,
            &[&atlas.bind_group_layout, &transform_bind_group_layout],
            wgpu::PolygonMode::Fill,
        );

        let pipeline_wireframe = VoxelPipeline::new(
            device,
            shader,
            cfg,
            &[&atlas.bind_group_layout, &transform_bind_group_layout],
            wgpu::PolygonMode::Line,
        );
        let mut world = Self {
            chunks: Vec::with_capacity(RENDER_DISTANCE as usize * 2),
            pipeline,
            pipeline_wireframe,
            atlas,
            wireframe: false,
            chunks_pos: HashSet::new(),
        };
        world.load_chunks(ChunkPos::from_world(camera.pos), device);
        let v_count = world
            .chunks
            .par_iter()
            .map(|c| c.buffer.vertex_buf.len())
            .sum::<usize>();
        info!("Vertices count: {}", v_count);
        world
    }

    pub fn on_update(&mut self, player_pos: Vec3<f32>, device: &wgpu::Device) {
        let player_chunk_pos = ChunkPos::from_world(player_pos);
        let mut dirty = false;

        for chunk in self.chunks.iter_mut() {
            let distance = chunk.pos - player_chunk_pos;
            let squared_distance = distance.x * distance.x + distance.z * distance.z;
            if squared_distance > RENDER_DISTANCE * RENDER_DISTANCE {
                dirty = true;
                chunk.loaded = false;
                self.chunks_pos.remove(&chunk.pos);
            }
        }
        if dirty {
            self.unload_chunks();
            let instant = std::time::Instant::now();
            self.load_chunks(player_chunk_pos, device);
            info!("Took {}ms to generate chunk", instant.elapsed().as_millis());
        }
    }

    pub fn unload_chunks(&mut self) {
        self.chunks.retain(|c| c.loaded);
    }

    pub fn load_chunks(&mut self, player_pos: ChunkPos, device: &wgpu::Device) {
        const DIST: i32 = RENDER_DISTANCE / 2;
        // new boundaries
        let start_x = player_pos.x - DIST;
        let end_x = player_pos.x + DIST;
        let start_z = player_pos.z - DIST;
        let end_z = player_pos.z + DIST;

        let chunks = (start_x..=end_x)
            .into_par_iter()
            .map(|x| {
                let chunks = (start_z..=end_z)
                    .into_par_iter()
                    .map(|z| ChunkPos::new(x, z))
                    .filter(|p| !self.chunks_pos.contains(p))
                    .map(|pos| Chunk::new(self, device, pos))
                    .collect::<Vec<_>>();
                return chunks;
            })
            .flatten()
            .collect::<Vec<_>>();

        self.chunks_pos.extend(chunks.iter().map(|c| c.pos));
        self.chunks.extend(chunks);
    }
}
