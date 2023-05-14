use self::vertex::Vertex;

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub num_elements: u32,
}

pub mod quad;
pub mod vertex;
