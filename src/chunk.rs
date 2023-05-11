use vek::Vec3;

use crate::{
    block::{Block, BlockId},
    renderer::{
        buffer::{compute_cube_indices, ChunkBuffer},
        mesh::{vertex::Vertex, Mesh},
    },
};

pub const CHUNK_Y_SIZE: usize = 200;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;

pub struct Chunk {
    pub blocks: Vec<Block>,
    pub buffer: ChunkBuffer,
    pub mesh: Mesh,
}

impl Chunk {
    pub fn new(device: &wgpu::Device) -> Self {
        let (blocks, mesh, indices) = Self::create_blocks();
        let buffer = ChunkBuffer::new(&device, mesh.vertices().to_vec(), indices);

        Self {
            blocks,
            mesh,
            buffer,
        }
    }
    pub fn create_blocks() -> (Vec<Block>, Mesh, Vec<u16>) {
        let mut blocks = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        for y in 0..CHUNK_Y_SIZE {
            for z in 0..CHUNK_Z_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    let block_pos = Vec3::new(x as f32, y as f32, z as f32);
                    let block = Block::new(BlockId::DIRT, [block_pos.x, block_pos.y, block_pos.z]);
                    vertices.extend_from_slice(&block.vertices());
                    indices.extend(compute_cube_indices(block.vertices().len() as usize));
                    blocks.push(block);
                }
            }
        }
        let mesh = Mesh::new(&vertices);
        (blocks, mesh, indices)
    }
}
