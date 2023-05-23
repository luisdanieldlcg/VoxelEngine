pub mod camera;

use std::time::Duration;

use crate::renderer::{buffer::Buffer, Renderer};
use bevy_ecs::world::World;
use vek::Vec3;

use self::camera::{Camera, CameraController, CameraUniform};

pub struct Scene {
    pub camera: Camera,
    pub camera_controller: camera::CameraController,
}

impl Scene {
    pub fn camera_pos(&self) -> Vec3<f32> {
        self.camera.pos
    }

    pub fn new(renderer: &Renderer, window_width: f32, window_height: f32) -> Self {
        let camera = Camera::new(window_width, window_height);

        let camera_controller = CameraController::new();
        Self {
            camera,
            camera_controller,
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

    pub fn tick(&mut self, queue: &wgpu::Queue, delta_time: Duration) {
        self.camera_controller.update(&mut self.camera, delta_time);

    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.on_resize(width, height);
    }
}
