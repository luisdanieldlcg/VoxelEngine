use std::{ops::Deref, vec};

use crate::{
    block::{Block, BlockId},
    renderer::{
        buffer::{compute_cube_indices, ChunkBuffer},
        mesh::{vertex::Vertex, ChunkMesh},
    },
};
use vek::Vec3;

pub const CHUNK_Y_SIZE: usize = 256;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;
pub const TOTAL_CHUNK_SIZE: usize = CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE;
pub const INITIAL_CHUNK_VERTICES: usize = (256 * 16 * 16) * 24;

pub struct Chunk {
    pub blocks: Vec<Vec<Vec<Block>>>,
    pub buffer: ChunkBuffer,
    pub mesh: ChunkMesh,
    pub pos: Vec3<i32>,
}

impl Chunk {
    pub fn new(device: &wgpu::Device, pos: Vec3<i32>) -> Self {
        let (blocks, mesh) = Self::generate_mesh(pos);
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
            pos,
        }
    }
    pub fn generate_mesh(offset: Vec3<i32>) -> (Vec<Vec<Vec<Block>>>, ChunkMesh) {
        let default = Block::new(BlockId::DIRT, Vec3::zero());
        let mut voxels: Vec<Vec<Vec<Block>>> =
            vec![vec![vec![default; CHUNK_Z_SIZE]; CHUNK_X_SIZE]; CHUNK_Y_SIZE];

        let mut vertices: Vec<Vertex> = Vec::new();

        let mut indices: Vec<u32> = Vec::new();

        for x in 0..CHUNK_X_SIZE {
            for y in 0..CHUNK_Y_SIZE {
                for z in 0..CHUNK_Z_SIZE {
                    let id = match y + 1 >= CHUNK_Y_SIZE {
                        true => BlockId::GRASS,
                        false => BlockId::DIRT,
                    };
                    let block = Block::new(id, Vec3::new(x as i32, y as i32, z as i32) + offset);
                    vertices.extend(block.iter_vertices());
                    voxels[y][x][z] = block;
                }
            }
        }
        indices.extend_from_slice(&compute_cube_indices(INITIAL_CHUNK_VERTICES));

        let mesh = ChunkMesh::new(vertices, indices);
        (voxels, mesh)
    }

    /// Checks if a given position is in bounds of the chunk
    pub fn is_pos_in_bounds(pos: Vec3<i32>) -> bool {
        if !pos.are_all_positive() {
            return false;
        }
        return pos.x < CHUNK_X_SIZE as i32
            && pos.y < CHUNK_Y_SIZE as i32
            && pos.z < CHUNK_Z_SIZE as i32;
    }
}
