use crate::block::{Block, BlockId};

pub const CHUNK_Y_SIZE: usize = 200;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;

pub struct Chunk {
    blocks: Vec<Block>,
    
}
impl Chunk {
    pub fn new() -> Self {
        let blocks = Self::create_blocks();
        Self { blocks }
    }

    pub fn create_blocks() -> Vec<Block> {
        let mut blocks = Vec::new();
        for y in 0..CHUNK_Y_SIZE {
            for z in 0..CHUNK_Z_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    let block_pos = [x as f32, y as f32, z as f32];
                    let block = Block::new(BlockId::DIRT, block_pos);
                    blocks.push(block);
                }
            }
        }
        blocks
    }
}
