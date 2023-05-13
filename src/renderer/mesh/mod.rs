// use vek::Vec3;

// use crate::block::BlockId;

// use self::vertex::Vertex;

pub mod quad;
pub mod vertex;

// type V = Vertex;

// #[derive(Debug)]
// pub struct Mesh {
//     vertices: Vec<Vertex>,
// }

// impl Mesh {
//     pub fn new(vertices: &[V]) -> Self {
//         Self {
//             vertices: Vec::from(vertices),
//         }
//     }

//     pub fn cube(block_id: &BlockId) -> Self {
//         Self::cube_offset(block_id, Vec3::zero())
//     }

//     pub fn cube_offset(block_id: &BlockId, offset: Vec3<f32>) -> Mesh {
//         let mut this = Mesh::new(&[]);

//         let (top, bottom, left, right, front, back) = match block_id {
//             BlockId::AIR => todo!(),
//             BlockId::DIRT => (2, 0, 1, 1, 1, 1),
//         };

//         // Block size = 1
//         let neg_x: f32 = -0.5 - offset.x;
//         let pos_x: f32 = 0.5 + offset.x;
//         let neg_y: f32 = -0.5 - offset.y;
//         let pos_y: f32 = 0.5 + offset.y;
//         let neg_z: f32 = -0.5 - offset.z;
//         let pos_z: f32 = 0.5 + offset.z;

//         // Top
//         // left -x
//         this.push_quad(Quad::new(
//             Vertex::new(neg_x, neg_y, neg_z, [0, 1], left),
//             Vertex::new(neg_x, pos_y, neg_z, [0, 0], left),
//             Vertex::new(neg_x, pos_y, pos_z, [1, 0], left),
//             Vertex::new(neg_x, neg_y, pos_z, [1, 1], left),
//         ));
//         // right +x
//         this.push_quad(Quad::new(
//             Vertex::new(pos_x, neg_y, pos_z, [0, 1], right),
//             Vertex::new(pos_x, pos_y, pos_z, [0, 0], right),
//             Vertex::new(pos_x, pos_y, neg_z, [1, 0], right),
//             Vertex::new(pos_x, neg_y, neg_z, [1, 1], right),
//         ));
//         // bottom -y
//         this.push_quad(Quad::new(
//             Vertex::new(pos_x, neg_y, neg_z, [0, 1], bottom),
//             Vertex::new(neg_x, neg_y, neg_z, [0, 0], bottom),
//             Vertex::new(neg_x, neg_y, pos_z, [1, 0], bottom),
//             Vertex::new(pos_x, neg_y, pos_z, [1, 1], bottom),
//         ));
//         // top +y
//         this.push_quad(Quad::new(
//             Vertex::new(pos_x, pos_y, pos_z, [0, 1], top),
//             Vertex::new(neg_x, pos_y, pos_z, [0, 0], top),
//             Vertex::new(neg_x, pos_y, neg_z, [1, 0], top),
//             Vertex::new(pos_x, pos_y, neg_z, [1, 1], top),
//         ));
//         // back -z
//         this.push_quad(Quad::new(
//             Vertex::new(neg_x, neg_y, neg_z, [0, 1], back),
//             Vertex::new(pos_x, neg_y, neg_z, [1, 1], back),
//             Vertex::new(pos_x, pos_y, neg_z, [1, 0], back),
//             Vertex::new(neg_x, pos_y, neg_z, [0, 0], back),
//         ));
//         // front +z
//         this.push_quad(Quad::new(
//             Vertex::new(neg_x, pos_y, pos_z, [0, 0], front),
//             Vertex::new(pos_x, pos_y, pos_z, [1, 0], front),
//             Vertex::new(pos_x, neg_y, pos_z, [1, 1], front),
//             Vertex::new(neg_x, neg_y, pos_z, [0, 1], front),
//         ));

//         this
//     }

//     pub fn push_quad(&mut self, quad: Quad) {
//         if V::INDEX_BUFFER_FORMAT.is_some() {
//             self.vertices.push(quad.v2);
//             self.vertices.push(quad.v1);
//             self.vertices.push(quad.v3);
//             self.vertices.push(quad.v4);
//             return;
//         }
//         // One half
//         self.vertices.push(quad.v3);
//         self.vertices.push(quad.v2);
//         self.vertices.push(quad.v3);
//         // Another half
//         self.vertices.push(quad.v3);
//         self.vertices.push(quad.v4);
//         self.vertices.push(quad.v3);
//     }

//     pub fn vertices(&self) -> &[V] {
//         &self.vertices
//     }
// }

// pub struct Quad {
//     v1: V,
//     v2: V,
//     v3: V,
//     v4: V,
// }

// impl Quad {
//     pub fn new(v3: V, v2: V, v1: V, v4: V) -> Self {
//         Self { v1, v2, v3, v4 }
//     }
// }
