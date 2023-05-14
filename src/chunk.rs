<<<<<<< HEAD
use std::println;

use vek::Vec3;
=======
use std::vec;

>>>>>>> 8006886 (Store blocks on a 3d array and try removing unseen faces)
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
pub const VERTICES: usize = (256 * 16 * 16) * 24;

pub struct Chunk {
    pub blocks: Vec<Vec<Vec<Block>>>,
    pub buffer: ChunkBuffer,
    pub mesh: ChunkMesh,
}

impl Chunk {
    pub fn new(device: &wgpu::Device) -> Self {
        let (blocks, mesh) = Self::create_blocks(Vec3::zero());
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
    pub fn create_blocks(offset: Vec3<i32>) -> (Vec<Vec<Vec<Block>>>, ChunkMesh) {
        // make a 3d array
        let default = Block::new(BlockId::AIR, Vec3::zero());
        let mut voxels: Vec<Vec<Vec<Block>>> =
            vec![vec![vec![default; CHUNK_Z_SIZE]; CHUNK_X_SIZE]; CHUNK_Y_SIZE];

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for y in 0..CHUNK_Y_SIZE {
            for z in 0..CHUNK_Z_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    let id = if y + 1 >= CHUNK_Y_SIZE {
                        BlockId::GRASS
                    } else {
                        BlockId::DIRT
                    };

                    let block = Block::new(id, Vec3::new(x as i32, y as i32, z as i32) + offset);
                    for quad in block.quads.iter() {
                        let dir_vec = quad.dir.to_vec();
                        let neighbor_pos = block.pos + dir_vec;

                        // Skip face checking if the neighbor out of bounds
                        if !Self::is_pos_in_bounds(neighbor_pos) {
                            vertices.extend_from_slice(&quad.vertices);
                            continue;
                        }

                        // neighbor block
                        let neighbor = &voxels[neighbor_pos.y as usize][neighbor_pos.x as usize]
                            [neighbor_pos.z as usize];

                        // render if neighbor is air
                        if neighbor.id == BlockId::AIR {
                            vertices.extend_from_slice(&quad.vertices);
                        }
                    }
                    voxels[y][x][z] = block;
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
        (voxels, mesh)
    }

    /// Checks if a given position is in bounds of the chunk
    pub fn is_pos_in_bounds(pos: Vec3<i32>) -> bool {
        if pos.x < 0 || pos.x > (CHUNK_X_SIZE as i32 - 1) {
            return false;
        }
        if pos.y < 0 || pos.y > (CHUNK_Y_SIZE as i32 - 1) {
            return false;
        }
        if pos.z < 0 || pos.z > (CHUNK_Z_SIZE as i32 - 1) {
            return false;
        }
        true
    }
}
