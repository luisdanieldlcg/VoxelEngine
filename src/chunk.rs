use crate::{
    block::{Block, BlockId},
    renderer::{
        buffer::ChunkBuffer,
        mesh::{vertex::Vertex, Mesh},
    },
};

pub const CHUNK_Y_SIZE: usize = 200;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;

pub struct Chunk {
    pub blocks: Vec<Block>,
    buffer: ChunkBuffer,
    pub mesh: Mesh,
}
impl Chunk {
    pub fn new(device: &wgpu::Device, indices: Vec<u16>) -> Self {
        let (blocks, mesh) = Self::create_blocks();
        let buffer = ChunkBuffer::new(&device, mesh.vertices().to_vec(), indices);
        Self {
            blocks,
            mesh,
            buffer,
        }
    }
    pub fn create_blocks() -> (Vec<Block>, Mesh) {
        let mut blocks = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        for y in 0..CHUNK_Y_SIZE {
            for z in 0..CHUNK_Z_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    let block_pos = [x as f32, y as f32, z as f32];
                    let block = Block::new(BlockId::DIRT, block_pos);
                    let mesh = Mesh::cube(&block.id());
                    let mesh_vertices = mesh
                        .vertices()
                        .iter()
                        .map(|v| v.offset(block_pos[0], block_pos[1], block_pos[2]))
                        .collect::<Vec<Vertex>>();
                    vertices.extend_from_slice(&mesh_vertices);
                    blocks.push(block);
                }
            }
        }
        let mesh = Mesh::new(&vertices);
        (blocks, mesh)
    }
}
