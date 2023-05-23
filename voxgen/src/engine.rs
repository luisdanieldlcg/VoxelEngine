use bevy_ecs::{schedule::Schedule, system::Commands, world::World};
use vek::Vec3;

use crate::{ecs::Transform, renderer::Renderer, window::Window};

pub struct VoxelEngine {
    pub(super) renderer: Renderer,
    pub window: Window,
    pub locked_input: bool,
    pub world: World,
}

impl VoxelEngine {
    pub fn on_key_pressed(&mut self, input: Option<winit::event::VirtualKeyCode>) {
        if let Some(key) = input {
            match key {
                winit::event::VirtualKeyCode::Escape => {
                    self.locked_input = !self.locked_input;
                    self.window.grab_cursor(!self.locked_input)
                }
                winit::event::VirtualKeyCode::F12 => self.renderer.toggle_wireframe(),
                _ => (),
            }
        }
    }

    pub fn setup_ecs(&mut self) {
        let mut schedule = Schedule::default();
        schedule.add_system(init_transform);
        schedule.run(&mut self.world);
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}

fn init_transform(mut commands: Commands) {
    commands.spawn(Transform { pos: Vec3::zero() });
}
