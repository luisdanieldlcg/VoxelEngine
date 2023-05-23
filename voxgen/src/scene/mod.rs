pub mod camera;

use std::time::Duration;

use crate::{ecs::Transform, renderer::Renderer};
use bevy_ecs::{schedule::Schedule, system::Commands};
use vek::Vec3;

use self::camera::{Camera, CameraController};

pub struct Scene {
    pub camera: Camera,
    pub camera_controller: camera::CameraController,
    pub world: bevy_ecs::world::World,
}

impl Scene {
    pub fn camera_pos(&self) -> Vec3<f32> {
        self.camera.pos
    }

    pub fn new(renderer: &Renderer, window_width: f32, window_height: f32) -> Self {
        let camera = Camera::new(window_width, window_height);
        let camera_controller = CameraController::new();
        let mut world = bevy_ecs::world::World::new();
        let mut schedule = Schedule::new();
        schedule.add_system(init_transform);
        schedule.run(&mut world);
        Self {
            camera,
            camera_controller,
            world,
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

    pub fn update(&mut self, delta_time: Duration) {
        let mut transform = self.world.query::<&mut Transform>();
        for mut transform in transform.iter_mut(&mut self.world) {
            transform.pos = self.camera.pos.map(|x| x as i32);
            log::info!("Transform: {:?}", transform.pos);
        }
        self.camera_controller.update(&mut self.camera, delta_time);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.on_resize(width, height);
    }
}

fn init_transform(mut command: Commands) {
    log::info!("Initializing transform");
    command.spawn(Transform { pos: Vec3::zero() });
}
