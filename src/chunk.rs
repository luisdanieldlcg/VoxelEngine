use crate::{
    block::{Block, BlockId},
    renderer::mesh::{vertex::Vertex, Mesh},
};

pub const CHUNK_Y_SIZE: usize = 200;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;

#[derive(Debug)]
pub struct Chunk {
    blocks: Vec<Block>,
    mesh: Mesh,
}
impl Chunk {
    pub fn new() -> Self {
        let (blocks, mesh) = Self::create_blocks();
        Self { blocks, mesh }
    }
    pub fn create_blocks() -> (Vec<Block>, Mesh) {
        let mut blocks = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..CHUNK_X_SIZE {
            for y in 0..CHUNK_Y_SIZE {
                for z in 0..CHUNK_Z_SIZE {
                    let block_pos = [x as f32, y as f32, z as f32];
                    let block = Block::new(BlockId::DIRT, block_pos);
                    let mesh = Mesh::cube(&block.id());

                    vertices.extend_from_slice(mesh.vertices());
                    blocks.push(block);
                }
            }
        }
        let mesh = Mesh::new(&vertices);
        (blocks, mesh)
    }
}
