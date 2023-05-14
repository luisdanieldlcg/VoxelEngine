
use egui::TextureId;

use super::vertex::Vertex;


pub struct BlockMesh {
    faces: BlockFace,
}

pub struct BlockFace {
    vertices: [Vertex; 4],
    dir: Direction,
    texture: TextureId,
}

impl BlockFace {
    pub fn new() -> Self {
        Self {
            vertices: todo!(),
            dir: todo!(),
            texture: todo!(),
        }
    }
}

pub enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}