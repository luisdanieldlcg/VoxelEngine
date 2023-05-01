use winit::event::KeyboardInput;

pub struct Camera {}

pub struct CameraController {}
impl CameraController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_keyboard_events(&mut self, input: &KeyboardInput) {}

    pub fn handle_mouse_events(&mut self) {}
}
