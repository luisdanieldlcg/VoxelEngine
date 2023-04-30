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
    pub fn len(&self) -> usize {
        self.len
    }
}
