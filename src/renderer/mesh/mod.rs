use self::vertex::{IVertex, Vertex};

pub mod vertex;

pub struct Mesh<V: IVertex> {
    vertices: Vec<V>,
}
impl<V: IVertex> Mesh<V> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn cube() -> Mesh<Vertex> {
        let mut this = Mesh::new();
        // -x
        this.push_quad(Quad::new(
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(-1.0, -1.0, 1.0),
        ));
        // +x
        this.push_quad(Quad::<Vertex>::new(
            Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),
            Vertex::new(1.0, 1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0),
        ));
        // -y
        this.push_quad(Quad::new(
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, 1.0),
        ));
        // +y
        this.push_quad(Quad::new(
            Vertex::new(1.0, 1.0, 1.0),
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0),
        ));
        // -z
        this.push_quad(Quad::new(
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0),
            Vertex::new(-1.0, 1.0, -1.0),
        ));
        // +z
        this.push_quad(Quad::new(
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),
            Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(-1.0, -1.0, 1.0),
        ));

        this
    }

    pub fn vertices(&self) -> &[V] {
        &self.vertices
    }
    pub fn push_triangle(&mut self, triangle: Triangle<V>) {
        self.vertices.push(triangle.v1);
        self.vertices.push(triangle.v2);
        self.vertices.push(triangle.v3);
    }

    pub fn push_quad(&mut self, quad: Quad<V>) {
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
}

pub struct Quad<V: IVertex> {
    v1: V,
    v2: V,
    v3: V,
    v4: V,
}

impl<V: IVertex> Quad<V> {
    pub fn new(v1: V, v2: V, v3: V, v4: V) -> Self {
        Self { v1, v2, v3, v4 }
    }
}

/// Represents the vertices of a triangle.
pub struct Triangle<V: IVertex> {
    v1: V,
    v2: V,
    v3: V,
}
impl<V: IVertex> Triangle<V> {
    pub fn new(v1: V, v2: V, v3: V) -> Self {
        Self { v1, v2, v3 }
    }
}
