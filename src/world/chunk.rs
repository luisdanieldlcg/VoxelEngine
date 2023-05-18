use std::{println, vec};

use crate::{
    block::BlockId,
    renderer::{
        buffer::{compute_cube_indices, ChunkBuffer},
        mesh::{quad::Quad, vertex::Vertex, ChunkMesh},
    },
};
use vek::Vec3;

pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_WIDTH: usize = 16;
pub const TOTAL_CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH;

pub struct Chunk {
    pub blocks: Vec<Vec<Vec<BlockId>>>,
    pub world_offset: ChunkPos,
    pub buffer: ChunkBuffer,
    pub mesh: ChunkMesh,
    pub loaded: bool,
}

impl Chunk {
    pub fn new(device: &wgpu::Device, pos: ChunkPos) -> Self {
        let instant = std::time::Instant::now();
        let blocks = Self::generate_data();
        let mesh = Self::generate_mesh(&blocks, &pos);
        let elapsed = instant.elapsed();
        println!("Took {}ms to generate chunk", elapsed.as_millis());

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
            world_offset: pos,
            loaded: true,
        }
    }
    pub fn generate_data() -> Vec<Vec<Vec<BlockId>>> {
        let mut blocks = vec![vec![vec![BlockId::AIR; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH];
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    let block_in_chunk = match y == CHUNK_HEIGHT - 1 {
                        true => BlockId::GRASS,
                        false => BlockId::DIRT,
                    };
                    blocks[x][y][z] = block_in_chunk;
                }
            }
        }
        blocks
    }
    pub fn generate_mesh(blocks: &Vec<Vec<Vec<BlockId>>>, pos: &ChunkPos) -> ChunkMesh {
        let mut vertices = Vec::with_capacity(TOTAL_CHUNK_SIZE);
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    // in memory block
                    let id: &BlockId = &blocks[x][y][z];

                    let x = x as i32;
                    let y = y as i32;
                    let z = z as i32;

                    // The position of the block in the chunk
                    let local_pos: Vec3<i32> = Vec3::new(x, y, z);

                    let world_pos = pos.to_world();

                    // Translate position to the world space
                    let translation = Vec3::new(
                        local_pos.x + world_pos.x,
                        local_pos.y,
                        local_pos.z + world_pos.z,
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
            return pos.x < CHUNK_WIDTH as i32
                && pos.y < CHUNK_HEIGHT as i32
                && pos.z < CHUNK_DEPTH as i32;
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

/// Represents the offset or indices of a chunk
/// relative to the world position.
///
/// Example:
///
/// If a chunk is 16 units wide and 16 units deep:
///
/// World Position: (32, 0, -128) -> ChunkPos: (2, 0, -8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl std::ops::Sub<ChunkPos> for ChunkPos {
    type Output = ChunkPos;

    fn sub(self, rhs: ChunkPos) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Add<ChunkPos> for ChunkPos {
    type Output = ChunkPos;

    fn add(self, rhs: ChunkPos) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}

impl ChunkPos {
    pub const ORIGIN: ChunkPos = ChunkPos::new(0, 0);

    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Returns the chunk pos at the given world pos
    pub fn from_world(pos: Vec3<f32>) -> Self {
        let x = (pos.x / CHUNK_WIDTH as f32).floor() as i32;
        let z = (pos.z / CHUNK_DEPTH as f32).floor() as i32;
        Self { x, z }
    }

    /// Returns the world pos of the current chunk.
    pub fn to_world(&self) -> Vec3<i32> {
        Vec3::new(self.x * CHUNK_WIDTH as i32, 0, self.z * CHUNK_DEPTH as i32)
    }
}
