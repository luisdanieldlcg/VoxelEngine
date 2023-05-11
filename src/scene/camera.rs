use std::time::Duration;

use vek::{Mat4, Vec3};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

type Point3 = Vec3<f32>;

pub struct Camera {
    pitch: f32,
    yaw: f32,
    pub pos: Point3,
    target: Vec3<f32>,
    pub fov_y_deg: f32,
    width: f32,
    height: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            pos: Vec3::new(10.0, 1.0, -5.0),
            target: Vec3::new(1.0, 1.0, 0.0),
            fov_y_deg: 45.0,
            width,
            height,
            near_plane: 0.1,
            far_plane: 400.0,
            yaw: -90.0, // Point torwards Z,
            pitch: 0.0,
        }
    }

    /// Translate the camera position
    pub fn translate(&mut self, offset: Vec3<f32>) {
        self.pos += offset;
    }

    /// Rotate around X and Y axis
    pub fn rotate(&mut self, x: f32, y: f32) {
        self.yaw += x;
        self.pitch += y;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let (yaw_sin, yaw_cos) = self.yaw.to_radians().sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.to_radians().sin_cos();
        // yaw_sin z goes negative for left handed coordinate system
        let rotation = Vec3::new(yaw_cos * pitch_cos, pitch_sin, -yaw_sin * pitch_cos).normalized();
        self.target = rotation;
    }

    pub fn on_resize(&mut self, new_width: f32, new_height: f32) {
        self.width = new_width;
        self.height = new_height;
    }

    pub fn update_proj(&self) -> Mat4<f32> {
        let proj = Mat4::perspective_fov_lh_no(
            self.fov_y_deg.to_radians(),
            self.width,
            self.height,
            self.near_plane,
            self.far_plane,
        );
        let model = Mat4::translation_3d(Vec3::new(10.0, 0.0 ,0.0));
        let view = Mat4::look_at_lh(self.pos, self.target + self.pos, Vec3::unit_y());
        proj * view * model * Mat4::scaling_3d(0.5)
    }
}

pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    mouse_dx: f32,
    mouse_dy: f32,
    pub speed: f32,
    pub sensitivity: f32,
}
impl CameraController {
    pub fn new() -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
            speed: 5.0,
            // TODO: find out why sensitivity has to be so high
            sensitivity: 70.0,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();
        let (yaw_sin, yaw_cos) = camera.yaw.to_radians().sin_cos();
        let forward = Vec3::new(yaw_cos, 0.0, -yaw_sin);
        let right = Vec3::new(yaw_sin, 0.0, yaw_cos);
        let multiplier = self.speed * dt;

        // Translation in x y z
        let dx = forward * (self.amount_forward - self.amount_backward) * multiplier;
        let dy = Vec3::new(0.0, (self.amount_up - self.amount_down) * multiplier, 0.0);
        let dz = right * (self.amount_left - self.amount_right) * multiplier;
        // Translate using WASD or arrow keys
        camera.translate(dx + dy + dz);

        let offset_x = self.mouse_dx * self.sensitivity * dt;
        let offset_y = self.mouse_dy * self.sensitivity * dt;
        camera.rotate(offset_x, -offset_y);

        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
    }

    pub fn handle_keyboard_events(&mut self, input: &KeyboardInput) {
        let amount = if input.state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        if let Some(key) = input.virtual_keycode {
            return match key {
                VirtualKeyCode::W | VirtualKeyCode::Up => {
                    self.amount_forward = amount;
                }
                VirtualKeyCode::S | VirtualKeyCode::Down => {
                    self.amount_backward = amount;
                }
                VirtualKeyCode::A | VirtualKeyCode::Left => {
                    self.amount_left = amount;
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.amount_right = amount;
                }
                VirtualKeyCode::Space => {
                    self.amount_up = amount;
                }
                VirtualKeyCode::LShift => {
                    self.amount_down = amount;
                }
                _ => (),
            };
        }
    }

    pub fn handle_mouse_events(&mut self, delta_x: f64, delta_y: f64) {
        self.mouse_dx = delta_x as f32;
        self.mouse_dy = delta_y as f32;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub transform: [[f32; 4]; 4],
}
impl CameraUniform {
    pub fn new(mat: vek::Mat4<f32>) -> Self {
        Self {
            transform: mat.into_col_arrays(),
        }
    }
    pub fn update(&mut self, camera: &Camera) {
        self.transform = camera.update_proj().into_col_arrays();
    }

    pub fn empty() -> Self {
        Self {
            transform: vek::Mat4::identity().into_col_arrays(),
        }
    }
    pub fn to_mat(&mut self) -> Mat4<f32> {
        Mat4::from_col_arrays(self.transform)
    }
}
