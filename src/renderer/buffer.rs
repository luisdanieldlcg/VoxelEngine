use bytemuck::Pod;
use wgpu::util::DeviceExt;

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
