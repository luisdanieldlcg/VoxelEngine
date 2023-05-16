use vek::Vec3;

use crate::{block::BlockId, direction::Direction};

use super::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Quad {
    pub vertices: [Vertex; 4],
    pub dir: Direction,
}

impl Quad {
    pub fn new(id: &BlockId, dir: Direction, offset: Vec3<i32>) -> Self {
        Self {
            vertices: dir.quad_vertices(id, offset),
            dir,
        }
    }
    pub fn generate_block_quads(id: &BlockId, offset: Vec3<i32>) -> [Quad; 6] {
        [
            Quad::new(id, Direction::TOP, offset),
            Quad::new(id, Direction::BOTTOM, offset),
            Quad::new(id, Direction::LEFT, offset),
            Quad::new(id, Direction::RIGHT, offset),
            Quad::new(id, Direction::FRONT, offset),
            Quad::new(id, Direction::BACK, offset),
        ]
    }
}

impl Direction {
    fn quad_vertices(&self, id: &BlockId, at: Vec3<i32>) -> [Vertex; 4] {
        let neg_x: f32 = -0.5;
        let pos_x: f32 = 0.5;
        let neg_y: f32 = -0.5;
        let pos_y: f32 = 0.5;
        let neg_z: f32 = -0.5;
        let pos_z: f32 = 0.5;

        match self {
            Direction::LEFT => [
                Vertex::quad(neg_x, neg_y, neg_z, at, [0, 1], &id, self),
                Vertex::quad(neg_x, pos_y, neg_z, at, [0, 0], &id, self),
                Vertex::quad(neg_x, pos_y, pos_z, at, [1, 0], &id, self),
                Vertex::quad(neg_x, neg_y, pos_z, at, [1, 1], &id, self),
            ],
            Direction::RIGHT => [
                Vertex::quad(pos_x, neg_y, pos_z, at, [0, 1], &id, self),
                Vertex::quad(pos_x, pos_y, pos_z, at, [0, 0], &id, self),
                Vertex::quad(pos_x, pos_y, neg_z, at, [1, 0], &id, self),
                Vertex::quad(pos_x, neg_y, neg_z, at, [1, 1], &id, self),
            ],
            Direction::BOTTOM => [
                Vertex::quad(pos_x, neg_y, neg_z, at, [0, 1], &id, self),
                Vertex::quad(neg_x, neg_y, neg_z, at, [0, 0], &id, self),
                Vertex::quad(neg_x, neg_y, pos_z, at, [1, 0], &id, self),
                Vertex::quad(pos_x, neg_y, pos_z, at, [1, 1], &id, self),
            ],
            Direction::TOP => [
                Vertex::quad(pos_x, pos_y, pos_z, at, [0, 1], &id, self),
                Vertex::quad(neg_x, pos_y, pos_z, at, [0, 0], &id, self),
                Vertex::quad(neg_x, pos_y, neg_z, at, [1, 0], &id, self),
                Vertex::quad(pos_x, pos_y, neg_z, at, [1, 1], &id, self),
            ],

            Direction::BACK => [
                Vertex::quad(neg_x, neg_y, neg_z, at, [0, 1], &id, self),
                Vertex::quad(pos_x, neg_y, neg_z, at, [1, 1], &id, self),
                Vertex::quad(pos_x, pos_y, neg_z, at, [1, 0], &id, self),
                Vertex::quad(neg_x, pos_y, neg_z, at, [0, 0], &id, self),
            ],
            Direction::FRONT => [
                Vertex::quad(neg_x, pos_y, pos_z, at, [0, 0], &id, self),
                Vertex::quad(pos_x, pos_y, pos_z, at, [1, 0], &id, self),
                Vertex::quad(pos_x, neg_y, pos_z, at, [1, 1], &id, self),
                Vertex::quad(neg_x, neg_y, pos_z, at, [0, 1], &id, self),
            ],
        }
    }
}
