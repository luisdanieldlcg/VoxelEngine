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

    pub fn cube(texture_id: u8) -> Mesh {
        let mut this = Mesh::new();
        // Originally: 
        // [0,0]
        // [0,1],
        // [1,1],
        // [1,0],
        // left
        this.push_quad(Quad::new(
            Vertex::new(-1, -1, -1, [1, 1], texture_id),
            Vertex::new(-1, 1, -1, [1, 0], texture_id),
            Vertex::new(-1, 1, 1, [0, 0], texture_id),
            Vertex::new(-1, -1, 1, [0, 1], texture_id),
        ));
        // right
        this.push_quad(Quad::new(
            Vertex::new(1, -1, 1, [1, 1], texture_id),
            Vertex::new(1, 1, 1, [1, 0], texture_id),
            Vertex::new(1, 1, -1, [0, 0], texture_id),
            Vertex::new(1, -1, -1, [0, 1], texture_id),
        ));
        // bottom
        this.push_quad(Quad::new(
            Vertex::new(1, -1, -1, [0, 0], texture_id),
            Vertex::new(-1, -1, -1, [0, 1], texture_id),
            Vertex::new(-1, -1, 1, [1, 1], texture_id),
            Vertex::new(1, -1, 1, [1, 0], texture_id),
        ));
        // top
        this.push_quad(Quad::new(
            Vertex::new(1, 1, 1, [0, 0], 1),
            Vertex::new(-1, 1, 1, [0, 1], 1),
            Vertex::new(-1, 1, -1, [1, 1], 1),
            Vertex::new(1, 1, -1, [1, 0], 1),
        ));
        // back
        this.push_quad(Quad::new(
            Vertex::new(-1, -1, -1, [0, 1], 0),
            Vertex::new(1, -1, -1, [0, 0], 0),
            Vertex::new(1, 1, -1, [1, 0], 0),
            Vertex::new(-1, 1, -1, [1, 1], 0),
        ));
        // front
        this.push_quad(Quad::new(
            Vertex::new(-1, 1, 1, [1, 1], 0),
            Vertex::new(1, 1, 1, [0, 1], 0),
            Vertex::new(1, -1, 1, [0, 0], 0),
            Vertex::new(-1, -1, 1, [1, 0], 0),
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
