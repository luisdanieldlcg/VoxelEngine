use bytemuck::Pod;
use wgpu::util::DeviceExt;

use super::mesh::vertex::Vertex;

pub struct ChunkBuffer {
    vertex_buf: Buffer<Vertex>,
    index_buf: Buffer<u16>,
    indices_len: u32,
}

impl ChunkBuffer {
    pub fn new(device: &wgpu::Device, vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self {
            vertex_buf: Buffer::new(device, wgpu::BufferUsages::VERTEX, &vertices),
            index_buf: Buffer::new(device, wgpu::BufferUsages::INDEX, &indices),
            indices_len: indices.len() as u32,
        }
    }
    pub fn vertex(&self) -> &Buffer<Vertex> {
        &self.vertex_buf
    }
    pub fn index(&self) -> &Buffer<u16> {
        &self.index_buf
    }
    pub fn indices(&self) -> u32 {
        self.indices_len
    }
    pub fn update(&mut self, queue: &wgpu::Queue, v_buf: &[Vertex], i_buf: &[u16]) {
        self.vertex_buf.update(&queue,&v_buf, 0);
        self.index_buf.update(&queue, &i_buf, 0);
        self.indices_len = i_buf.len() as u32;
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
pub fn compute_cube_indices(vertices: usize) -> Vec<u16> {
    let indices = [0, 1, 2, 2, 1, 3]
        .iter()
        .cycle()
        .copied()
        .take(vertices / 4 * 6)
        .enumerate()
        .map(|(i, b)| (i / 6 * 4 + b) as u16)
        .collect::<Vec<_>>();
    indices
}
pub fn create_cube_index_buffer(device: &wgpu::Device) -> Buffer<u16> {
    let vertices = 24;
    let indices = compute_cube_indices(vertices);
    Buffer::new(&device, wgpu::BufferUsages::INDEX, &indices)
}
