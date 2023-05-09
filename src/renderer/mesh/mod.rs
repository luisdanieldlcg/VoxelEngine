use vek::Vec3;

use crate::block::BlockId;

use self::vertex::Vertex;

pub mod vertex;

type V = Vertex;

pub struct Mesh {
    vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn cube(block_id: &BlockId) -> Mesh {
        let mut this = Mesh::new();

        let (top, bottom, left, right, front, back) = match block_id {
            BlockId::AIR => todo!(),
            BlockId::DIRT => (2, 0, 1, 1, 1, 1),
        };

        // left -x
        this.push_quad(Quad::new(
            Vertex::new(-1, -1, -1, [0, 1], left),
            Vertex::new(-1, 1, -1, [0, 0], left),
            Vertex::new(-1, 1, 1, [1, 0], left),
            Vertex::new(-1, -1, 1, [1, 1], left),
        ));
        // right +x
        this.push_quad(Quad::new(
            Vertex::new(1, -1, 1, [0, 1], right),
            Vertex::new(1, 1, 1, [0, 0], right),
            Vertex::new(1, 1, -1, [1, 0], right),
            Vertex::new(1, -1, -1, [1, 1], right),
        ));
        // bottom -y
        this.push_quad(Quad::new(
            Vertex::new(1, -1, -1, [0, 1], bottom),
            Vertex::new(-1, -1, -1, [0, 0], bottom),
            Vertex::new(-1, -1, 1, [1, 0], bottom),
            Vertex::new(1, -1, 1, [1, 1], bottom),
        ));
        // top +y
        this.push_quad(Quad::new(
            Vertex::new(1, 1, 1, [0, 1], top),
            Vertex::new(-1, 1, 1, [0, 0], top),
            Vertex::new(-1, 1, -1, [1, 0], top),
            Vertex::new(1, 1, -1, [1, 1], top),
        ));
        // back -z
        this.push_quad(Quad::new(
            Vertex::new(-1, -1, -1, [0, 1], back),
            Vertex::new(1, -1, -1, [1, 1], back),
            Vertex::new(1, 1, -1, [1, 0], back),
            Vertex::new(-1, 1, -1, [0, 0], back),
        ));
        // front +z
        this.push_quad(Quad::new(
            Vertex::new(-1, 1, 1, [0, 0], front),
            Vertex::new(1, 1, 1, [1, 0], front),
            Vertex::new(1, -1, 1, [1, 1], front),
            Vertex::new(-1, -1, 1, [0, 1], front),
        ));

        this
    }

    pub fn push_quad(&mut self, quad: Quad) {
        if V::INDEX_BUFFER_FORMAT.is_some() {
            self.vertices.push(quad.v2);
            self.vertices.push(quad.v3);
            self.vertices.push(quad.v1);
            self.vertices.push(quad.v4);
            return;
        }
        // One half
        self.vertices.push(quad.v1);
        self.vertices.push(quad.v2);
        self.vertices.push(quad.v3);
        // Another half
        self.vertices.push(quad.v3);
        self.vertices.push(quad.v4);
        self.vertices.push(quad.v1);
    }

    pub fn vertices(&self) -> &[V] {
        &self.vertices
    }
}

pub struct Quad {
    v1: V,
    v2: V,
    v3: V,
    v4: V,
}

impl Quad {
    pub fn new(v1: V, v2: V, v3: V, v4: V) -> Self {
        Self { v1, v2, v3, v4 }
    }
}

/// Represents the vertices of a triangle.
pub struct Triangle {
    v1: V,
    v2: V,
    v3: V,
}
impl Triangle {
    pub fn new(v1: V, v2: V, v3: V) -> Self {
        Self { v1, v2, v3 }
    }
}

pub fn should_render_quad_at(block_pos: Vec3<u8>, dir: Vec3<u8>) {
    // Determine if the quad should be rendered by checking neighboring blocks.

    let neighbor_pos = block_pos + dir;
}
