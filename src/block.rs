#[derive(Debug)]
pub enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    BACK,
    FRONT,
}

#[derive(Debug)]
pub enum BlockId {
    AIR = 0,
    DIRT = 1,
}

#[derive(Debug)]
pub struct Block {
    id: BlockId,
}

impl Block {
    pub fn new(id: BlockId) -> Self {
        Self { id }
    }
    pub fn id(&self) -> &BlockId {
        &self.id
    }
}
