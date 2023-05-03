use std::time::Instant;

use vek::{Mat4, Vec3};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use super::uniforms::TransformUniform;

pub struct Camera {
    pub pos: Vec3<f32>,
    pub target: Vec3<f32>,
    pub up: Vec3<f32>,
    pub fov_y_deg: f32,
    pub width: f32,
    pub height: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    time: std::time::Instant,
}
pub struct CameraController {
    // camera: Camera,
    is_forward: bool,
    is_backward: bool,
    is_left: bool,
    is_right: bool,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 10.0),
            target: Vec3::zero(),
            up: Vec3::unit_y(),
            fov_y_deg: 20.0,
            width,
            height,
            near_plane: 0.1,
            far_plane: 100.0,
            time: Instant::now(),
        }
    }
    pub fn update_proj(&self) -> Mat4<f32> {
        let view = Mat4::<f32>::look_at_rh(self.pos, self.target, self.up);
        let projection = Mat4::perspective_fov_rh_zo(
            self.fov_y_deg.to_radians(),
            self.width,
            self.height,
            self.near_plane,
            self.far_plane,
        );
        let rot = Mat4::rotation_z(self.time.elapsed().as_secs_f32() * 15.0f32.to_radians());
        rot * projection * view
    }
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            // camera: Camera::new(),
            is_forward: false,
            is_backward: false,
            is_left: false,
            is_right: false,
        }
    }

    pub fn update(&mut self, camera: &mut Camera) -> TransformUniform {
        let proj = camera.update_proj();
        let forward = camera.target - camera.pos;

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
