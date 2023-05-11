use vek::Vec3;

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
    pos: Vec3<f32>,
}

impl Block {
    pub fn new(id: BlockId, pos: [f32; 3]) -> Self {
        Self {
            id,
            pos: Vec3::from(pos),
        }
    }
    pub fn id(&self) -> &BlockId {
        &self.id
    }
    pub fn pos(&self) -> &Vec3<f32> {
        &self.pos
    }
}
