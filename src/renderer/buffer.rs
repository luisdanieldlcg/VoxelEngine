use bytemuck::Pod;
use wgpu::util::DeviceExt;

use super::mesh::vertex::Vertex;

pub struct ChunkBuffer {
    pub vertex_buf: Buffer<Vertex>,
    pub index_buf: Buffer<u32>,
    pub indices_len: u32,
}

impl ChunkBuffer {
    pub fn new(
        device: &wgpu::Device,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        num_elements: u32,
    ) -> Self {
        Self {
            vertex_buf: Buffer::new(
                device,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                &vertices,
            ),
            index_buf: Buffer::new(
                device,
                wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                &indices,
            ),
            indices_len: num_elements,
        }
    }
}

pub struct Buffer<T: Copy + bytemuck::Pod> {
    pub buf: wgpu::Buffer,
    len: usize,
    phantom_data: std::marker::PhantomData<T>,
}

impl<T: Copy + Pod> Buffer<T> {
    pub fn new(device: &wgpu::Device, usage: wgpu::BufferUsages, data: &[T]) -> Self {
        Self {
            buf: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage,
            }),
            len: data.len(),
            phantom_data: std::marker::PhantomData,
        }
    }
    pub fn update(&self, queue: &wgpu::Queue, data: &[T], offset: usize) {
        if data.is_empty() {
            return;
        }
        queue.write_buffer(
            &self.buf,
            offset as u64 * std::mem::size_of::<T>() as u64,
            bytemuck::cast_slice(data),
        )
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
pub fn compute_cube_indices(vertices: usize) -> Vec<u32> {
    let indices = [0, 1, 2, 2, 3, 0]
        .iter()
        .cycle()
        .copied()
        .take(vertices / 4 * 6)
        .enumerate()
        .map(|(i, b)| (i / 6 * 4 + b) as u32)
        .collect::<Vec<_>>();
    indices
}
pub fn create_cube_index_buffer(device: &wgpu::Device) -> Buffer<u32> {
    let vertices = 24;
    let indices = compute_cube_indices(vertices);
    Buffer::new(&device, wgpu::BufferUsages::INDEX, &indices)
}
