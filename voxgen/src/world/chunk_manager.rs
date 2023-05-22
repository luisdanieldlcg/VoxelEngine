use super::chunk::{Chunk, ChunkPos};
use crate::renderer::world::RENDER_DISTANCE;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub struct ChunkManager {
    chunks: Vec<Chunk>,
    positions: HashSet<ChunkPos>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunks: Vec::with_capacity(25),
            positions: HashSet::new(),
        }
    }

    pub fn tick(&mut self, player_pos: ChunkPos, device: &wgpu::Device) {
        let mut dirty = false;
        for chunk in self.chunks.iter_mut() {
            let distance = chunk.pos - player_pos;
            let squared_distance = distance.x * distance.x + distance.z * distance.z;
            if squared_distance > RENDER_DISTANCE * RENDER_DISTANCE {
                dirty = true;
                chunk.loaded = false;
                self.positions.remove(&chunk.pos);
            }
        }
        if dirty {
            self.unload_chunks();
            let instant = std::time::Instant::now();
            self.load_chunks(player_pos, device);
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
                    .filter(|p| !self.positions.contains(p))
                    .map(|pos| Chunk::new(device, pos))
                    .collect::<Vec<_>>();
                return chunks;
            })
            .flatten()
            .collect::<Vec<_>>();

        self.positions.extend(chunks.iter().map(|c| c.pos));
        self.chunks.extend(chunks);
    }

    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }
}
