use vek::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockId {
    AIR = 0,
    DIRT = 1,
    GRASS = 2,
    STONE = 3,
}
impl BlockId {
    pub fn is_air(&self) -> bool {
        self == &BlockId::AIR
    }
}
#[derive(Debug, Clone)]
pub struct Block {
    pub id: BlockId,
    pub pos: Vec3<i32>,
}

impl Block {
    pub fn new(id: BlockId, pos: Vec3<i32>) -> Self {
        Self { id, pos }
    }

    pub fn id(&self) -> &BlockId {
        &self.id
    }
    pub fn pos(&self) -> &Vec3<i32> {
        &self.pos
    }
}
