use crate::{renderer::Renderer, window::Window};

pub struct VoxelEngine {
    pub(super) renderer: Renderer,
    pub window: Window,
    pub locked_input: bool,
}

impl VoxelEngine {
    pub fn on_key_pressed(&mut self, input: Option<winit::event::VirtualKeyCode>) {
        if let Some(key) = input {
            match key {
                winit::event::VirtualKeyCode::Escape => self.locked_input = !self.locked_input,
                winit::event::VirtualKeyCode::F12 => self.renderer.toggle_wireframe(),
                _ => (),
            }
        }
    }
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
