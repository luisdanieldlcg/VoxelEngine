use std::time::Duration;

use crate::renderer::buffer::Buffer;
use vek::Vec3;

use self::camera::{Camera, CameraController, CameraUniform};

pub mod camera;

pub struct Scene {
    camera: Camera,
    camera_uniform: CameraUniform,
    uniform_buf: Buffer<CameraUniform>,
    pub camera_controller: camera::CameraController,
    pub transform_bind_group: wgpu::BindGroup,
}

impl Scene {
    pub fn camera_pos(&self) -> Vec3<f32> {
        self.camera.pos
    }

    pub fn new(
        device: &wgpu::Device,
        window_width: f32,
        window_height: f32,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let camera = Camera::new(window_width, window_height);
        let mut camera_uniform: CameraUniform = CameraUniform::empty();
        camera_uniform.update(&camera);

        let transform_buffer = Buffer::new(
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[camera_uniform],
        );

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.buf.as_entire_binding(),
            }],
        });
        let camera_controller = CameraController::new();
        Self {
            camera,
            camera_uniform,
            uniform_buf: transform_buffer,
            camera_controller,
            transform_bind_group,
        }
    }

    pub fn handle_input_events(&mut self, event: &winit::event::Event<()>) {
        if let winit::event::Event::WindowEvent { event, .. } = event {
            if let winit::event::WindowEvent::KeyboardInput { input, .. } = event {
                self.camera_controller.handle_keyboard_events(input);
            }
        }

        if let winit::event::Event::DeviceEvent {
            event: winit::event::DeviceEvent::MouseMotion { delta },
            ..
        } = event
        {
            self.camera_controller.handle_mouse_events(delta.0, delta.1);
        }
    }

    pub fn update_scene(&mut self, queue: &wgpu::Queue, delta_time: Duration) {
        self.camera_controller.update(&mut self.camera, delta_time);
        self.camera_uniform.update(&self.camera);
        self.uniform_buf.update(queue, &[self.camera_uniform], 0);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.on_resize(width, height);
    }
}
