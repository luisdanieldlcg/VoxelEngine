use std::{println, vec};

use crate::{
    block::{Block, BlockId},
    renderer::{
        buffer::{compute_cube_indices, ChunkBuffer},
        mesh::{quad::Quad, vertex::Vertex, ChunkMesh},
    },
};
use vek::{Vec2, Vec3};

pub const CHUNK_Y_SIZE: usize = 256;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;
pub const TOTAL_CHUNK_SIZE: usize = CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE;
pub const INITIAL_CHUNK_VERTICES: usize = (256 * 16 * 16) * 24;

pub struct Chunk {
    pub blocks: Vec<Vec<Vec<BlockId>>>,
    pub pos: Vec3<i32>,
    pub buffer: ChunkBuffer,
    pub mesh: ChunkMesh,
}

impl Chunk {
    pub fn new(device: &wgpu::Device, pos: Vec3<i32>) -> Self {
        let blocks = Self::generate_data();
        let mesh = Self::generate_mesh(&blocks, pos);
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
    pub fn generate_data() -> Vec<Vec<Vec<BlockId>>> {
        let mut blocks = vec![vec![vec![BlockId::AIR; CHUNK_X_SIZE]; CHUNK_Y_SIZE]; CHUNK_Z_SIZE];
        for x in 0..CHUNK_X_SIZE {
            for y in 0..CHUNK_Y_SIZE {
                for z in 0..CHUNK_Z_SIZE {
                    let block_in_chunk = match y == CHUNK_Y_SIZE - 1 {
                        true => BlockId::GRASS,
                        false => BlockId::DIRT,
                    };
                    blocks[x][y][z] = block_in_chunk;
                }
            }
        }
        blocks
    }
    pub fn generate_mesh(blocks: &Vec<Vec<Vec<BlockId>>>, world_pos: Vec3<i32>) -> ChunkMesh {
        let mut vertices = Vec::new();
        for x in 0..CHUNK_X_SIZE {
            for y in 0..CHUNK_Y_SIZE {
                for z in 0..CHUNK_Z_SIZE {
                    // in memory block
                    let id: &BlockId = &blocks[x][y][z];

                    let x = x as i32;
                    let y = y as i32;
                    let z = z as i32;
                    
                    // The position of the block in the chunk
                    let local_pos: Vec3<i32> = Vec3::new(x, y, z);
                    // Translate position to the world space
                    let translation = Vec3::new(
                        local_pos.x + world_pos.x,
                        local_pos.x + world_pos.y,
                        local_pos.x + world_pos.z,
                    );
                    // Create quads at that location
                    Quad::generate_block_quads(&id, translation)
                        .iter()
                        .for_each(|quad| create_quad(blocks, local_pos, &mut vertices, quad));
                }
            }
        }
        let indices = compute_cube_indices(vertices.len());
        ChunkMesh::new(vertices, indices)
    }

    /// Checks if a given position is in bounds of the chunk
    pub fn is_pos_in_bounds(pos: Vec3<i32>) -> bool {
        if pos.x >= 0 && pos.y >= 0 && pos.z >= 0 {
            return pos.x < CHUNK_X_SIZE as i32
                && pos.y < CHUNK_Y_SIZE as i32
                && pos.z < CHUNK_Z_SIZE as i32;
        }
        false
    }
}

fn create_quad(
    blocks: &Vec<Vec<Vec<BlockId>>>,
    local_pos: Vec3<i32>,
    vertices: &mut Vec<Vertex>,
    quad: &Quad,
) {
    let normal = quad.dir.normalized();
    // The position of the neighbor block in chunk
    let neighbor = local_pos + normal;
    if !Chunk::is_pos_in_bounds(neighbor) {
        vertices.extend(quad.vertices);
        return;
    }
    let neighbor_block = &blocks[neighbor.x as usize][neighbor.y as usize][neighbor.z as usize];
    if neighbor_block.is_air() {
        vertices.extend(quad.vertices);
    }
}
