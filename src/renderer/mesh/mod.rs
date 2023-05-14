use self::vertex::Vertex;

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub num_elements: u32,
}
impl ChunkMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            num_elements: indices.len() as u32,
            indices,
        }
    }
}
pub mod quad;
pub mod vertex;
