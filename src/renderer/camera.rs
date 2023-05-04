use std::time::{Duration, Instant};

use vek::{serde::__private::de, transform, Mat4, Vec3};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use super::uniforms::TransformUniform;

type Point3 = Vec3<f32>;

pub struct Camera {
    pitch: f32,
    yaw: f32,
    pub pos: Point3,
    pub target: Vec3<f32>,
    pub up: Vec3<f32>,
    pub fov_y_deg: f32,
    pub width: f32,
    pub height: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    dt: std::time::Instant,
}
pub struct CameraController {
    // camera: Camera,
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    mouse_dx: f32,
    mouse_dy: f32,
    speed: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 10.0),
            target: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::unit_y(),
            fov_y_deg: 45.0,
            width,
            height,
            near_plane: 0.1,
            far_plane: 100.0,
            dt: Instant::now(),
            yaw: -90.0, // Point torwards -Z,
            pitch: 20.0,
        }
    }
    pub fn update_proj(&self) -> Mat4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let new_rotation =
            Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalized();
        let view = Mat4::<f32>::look_at_rh(self.pos, new_rotation, self.up);
        let projection = Mat4::perspective_fov_rh_zo(
            self.fov_y_deg.to_radians(),
            self.width,
            self.height,
            self.near_plane,
            self.far_plane,
        );
        projection * view
    }
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            // camera: Camera::new(),
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
            speed: 4.0,
            sensitivity: 0.4,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, dt: Duration) -> TransformUniform {
        let dt = dt.as_secs_f32();
        let speed = self.speed;
        let sensitivity = self.sensitivity;
        let (yaw_sin, yaw_cos) = camera.yaw.to_radians().sin_cos();
        let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalized();
        let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalized();
        camera.pos += forward * speed * (self.amount_forward - self.amount_backward) * speed * dt;
        camera.pos += right * speed * (self.amount_right - self.amount_left) * speed * dt;

        camera.pos.y += (self.amount_up - self.amount_down) * speed * dt;

        camera.yaw += self.mouse_dx  * sensitivity * dt * 12.0;
        camera.pitch += - self.mouse_dy  * sensitivity * dt * 12.0;

        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;

        if camera.pitch > 89.0 {
            camera.pitch = 89.0;
        }
        if camera.pitch < -89.0 {
            camera.pitch = -89.0;
        }

        let proj = camera.update_proj();

        TransformUniform::new(proj)
    }

    pub fn handle_keyboard_events(&mut self, input: &KeyboardInput) -> bool {
        let amount = if input.state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        if let Some(key) = input.virtual_keycode {
            return match key {
                VirtualKeyCode::W | VirtualKeyCode::Up => {
                    self.amount_forward = amount;
                    true
                }
                VirtualKeyCode::S | VirtualKeyCode::Down => {
                    self.amount_backward = amount;
                    true
                }
                VirtualKeyCode::A | VirtualKeyCode::Left => {
                    self.amount_left = amount;
                    true
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.amount_right = amount;
                    true
                }
                VirtualKeyCode::Space => {
                    self.amount_up = amount;
                    true
                }
                VirtualKeyCode::LShift => {
                    self.amount_down = amount;
                    true
                }
                _ => false,
            };
        }
        false
    }

    pub fn handle_mouse_events(&mut self, delta_x: f64, delta_y: f64) {
        self.mouse_dx = delta_x as f32;
        self.mouse_dy = delta_y as f32;
    }
}
