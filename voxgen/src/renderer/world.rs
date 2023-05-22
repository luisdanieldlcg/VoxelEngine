use crate::{
    scene::camera::Camera,
    world::{chunk::ChunkPos, chunk_manager::ChunkManager},
};
use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use vek::Vec3;

use super::{atlas::Atlas, pipelines::voxel::VoxelPipeline, Renderable};

pub const RENDER_DISTANCE: i32 = 4;

pub struct WorldRenderer {
    chunk_manager: ChunkManager,
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

        for chunk in self.chunk_manager.chunks() {
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
        cfg: &wgpu::SurfaceConfiguration,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../../assets/shaders/cube.wgsl"));
        let texture_atlas = include_bytes!("../../../assets/atlas.png");
        let atlas = Atlas::new(texture_atlas, &device, &queue);
        let pipeline = VoxelPipeline::new(
            device,
            &shader,
            cfg,
            &[&atlas.bind_group_layout, &transform_bind_group_layout],
            wgpu::PolygonMode::Fill,
        );

        let pipeline_wireframe = VoxelPipeline::new(
            device,
            &shader,
            cfg,
            &[&atlas.bind_group_layout, &transform_bind_group_layout],
            wgpu::PolygonMode::Line,
        );
        let mut world = Self {
            chunk_manager: ChunkManager::new(),
            pipeline,
            pipeline_wireframe,
            atlas,
            wireframe: false,
        };
        world
            .chunk_manager
            .load_chunks(ChunkPos::from_world(camera.pos), device);
        let v_count = world
            .chunk_manager
            .chunks()
            .par_iter()
            .map(|c| c.buffer.vertex_buf.len())
            .sum::<usize>();
        info!("Vertices count: {}", v_count);
        world
    }

    pub fn tick(&mut self, player_pos: Vec3<f32>, device: &wgpu::Device) {
        let player_chunk_pos = ChunkPos::from_world(player_pos);
        self.chunk_manager.tick(player_chunk_pos, device);
    }
}
