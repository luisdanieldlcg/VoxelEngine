use vek::{Mat4, Vec3};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use super::uniforms::TransformUniform;

pub struct Camera {
    pos: Vec3<f32>,
    target: Vec3<f32>,
    up: Vec3<f32>,
}
pub struct CameraController {
    camera: Camera,
    is_forward: bool,
    is_backward: bool,
    is_left: bool,
    is_right: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 10.0),
            target: Vec3::zero(),
            up: Vec3::unit_y(),
        }
    }
    pub fn update_proj(&self) -> Mat4<f32> {
        let view = Mat4::<f32>::look_at_rh(self.pos, self.target, self.up);
        let projection = Mat4::perspective_fov_rh_zo(0.5, 800.0, 600.0, 0.1, 100.0);
        projection * view
    }
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            is_forward: false,
            is_backward: false,
            is_left: false,
            is_right: false,
        }
    }

    pub fn update(&mut self) -> TransformUniform {
        let proj = self.camera.update_proj();
        let forward = self.camera.target - self.camera.pos;

        TransformUniform::new(proj)
    }

    pub fn handle_keyboard_events(&mut self, input: &KeyboardInput) -> bool {
        let pressed = input.state == ElementState::Pressed;
        if let Some(key) = input.virtual_keycode {
            return match key {
                VirtualKeyCode::W | VirtualKeyCode::Up => {
                    self.is_forward = pressed;
                    true
                }
                VirtualKeyCode::A | VirtualKeyCode::Left => {
                    self.is_left = pressed;
                    true
                }
                VirtualKeyCode::S | VirtualKeyCode::Down => {
                    self.is_backward = pressed;
                    true
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.is_right = pressed;
                    true
                }
                _ => false,
            };
        }
        false
    }

    pub fn handle_mouse_events(&mut self) {}
}
