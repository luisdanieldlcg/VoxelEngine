use super::vertex::IVertex;

pub struct Mesh<V: IVertex> {
    vertices: Vec<V>,
}
impl<V: IVertex> Mesh<V> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
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
        self.vertices.push(quad.v1);
        self.vertices.push(quad.v2);
        self.vertices.push(quad.v3);

        // Tri 2
        self.vertices.push(quad.v1);
        self.vertices.push(quad.v4);
        self.vertices.push(quad.v3);
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
