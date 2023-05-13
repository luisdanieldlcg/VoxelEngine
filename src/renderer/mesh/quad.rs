use vek::Vec3;

use crate::block::{BlockId, Direction};

use super::vertex::Vertex;

#[derive(Debug)]
pub struct Quad {
    pub vertices: [Vertex; 4],
    pub dir: Direction,
}

impl Quad {
    pub fn new(id: &BlockId, dir: Direction, offset: Vec3<f32>) -> Self {
        Self {
            vertices: dir.quad_vertices(id, offset),
            dir,
        }
    }
    pub fn create_block_quads(id: &BlockId, offset: Vec3<f32>) -> [Quad; 6] {
        [
            Quad::new(id, Direction::TOP, offset),
            Quad::new(id, Direction::BOTTOM, offset),
            Quad::new(id, Direction::LEFT, offset),
            Quad::new(id, Direction::RIGHT, offset),
            Quad::new(id, Direction::FRONT, offset),
            Quad::new(id, Direction::BACK, offset),
        ]
    }
    pub fn get_indices(&self, i: u16) -> [u16; 6] {
        let displacement = i * 4;
        [
            0 + displacement,
            1 + displacement,
            2 + displacement,
            2 + displacement,
            3 + displacement,
            0 + displacement,
        ]
    }
}

impl Direction {
    fn quad_vertices(&self, id: &BlockId, at: Vec3<f32>) -> [Vertex; 4] {
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
