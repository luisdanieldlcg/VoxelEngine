use vek::Vec3;

use crate::renderer::mesh::{quad::Quad, vertex::Vertex};

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
    GRASS = 2,
    STONE = 3,
}

#[derive(Debug)]
pub struct Block {
    pub id: BlockId,
    pub pos: Vec3<f32>,
    pub quads: [Quad; 6],
}

impl Block {
    pub fn new(id: BlockId, pos: Vec3<f32>) -> Self {
        let quads = Quad::create_block_quads(&id, pos);
        Self { id, pos, quads }
    }
    pub fn vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        for quad in self.quads.iter() {
            vertices.extend_from_slice(&quad.vertices);
        }
        vertices
    }

    pub fn id(&self) -> &BlockId {
        &self.id
    }
    pub fn pos(&self) -> &Vec3<f32> {
        &self.pos
    }

    pub fn update(&mut self, offset: Vec3<f32>) {
        self.quads = Quad::create_block_quads(&self.id, offset);
    }
}
