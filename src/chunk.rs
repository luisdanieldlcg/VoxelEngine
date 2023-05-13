use std::println;

use vek::Vec3;

use crate::{
    block::{Block, BlockId},
    renderer::{
        buffer::{compute_cube_indices, ChunkBuffer},
        mesh::vertex::Vertex,
    },
};

pub const CHUNK_Y_SIZE: usize = 200;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;
pub const VERTICES: usize = 1228800;

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub num_elements: u32,
}
pub struct Chunk {
    pub blocks: Vec<Block>,
    pub buffer: ChunkBuffer,
    pub mesh: ChunkMesh,
}

impl Chunk {
    pub fn new(device: &wgpu::Device) -> Self {
        let (blocks, mesh) = Self::create_blocks([0.0, 0.0, 0.0].into());
        let buffer = ChunkBuffer::new(
            &device,
            mesh.vertices.clone(),
            mesh.indices.clone(),
            mesh.num_elements,
        );
        Self {
            blocks,
            buffer,
            mesh,
        }
    }
    pub fn create_blocks(offset: Vec3<f32>) -> (Vec<Block>, ChunkMesh) {
        let mut blocks = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        for y in 0..CHUNK_Y_SIZE {
            for z in 0..CHUNK_Z_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    let x = x as f32;
                    let y = y as f32;
                    let z = z as f32;
                    let block = Block::new(BlockId::DIRT, Vec3::new(x, y, z) + offset);
                    for quad in block.quads.iter() {
                        vertices.extend_from_slice(&quad.vertices);
                    }  
                    blocks.push(block);
                }        
            }
        }
        indices.extend_from_slice(&compute_cube_indices(VERTICES));
        let num_elements = indices.len() as u32;

        let mesh = ChunkMesh {
            vertices,
            indices,
            num_elements,
        };
        (blocks, mesh)
    }
}
