use vek::Vec3;

use crate::renderer::mesh::{vertex::Vertex, Mesh};

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
    mesh: Mesh,
}

impl Block {
    pub fn new(id: BlockId, pos: [f32; 3]) -> Self {
        Self {
            mesh: Mesh::cube(&id),
            pos: Vec3::from(pos),
            id,
        }
    }

    pub fn id(&self) -> &BlockId {
        &self.id
    }
    pub fn pos(&self) -> &Vec3<f32> {
        &self.pos
    }
    pub fn vertices(&self) -> &[Vertex] {
        self.mesh.vertices()
    }

}
