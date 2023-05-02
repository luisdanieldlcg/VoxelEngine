use egui::{DragValue, Grid};
use egui_winit_platform::{Platform, PlatformDescriptor};
use vek::Vec3;

use crate::renderer::camera::Camera;

pub struct EguiInstance {
    pub platform: Platform,
    pub(super) state: UIState,
}

pub struct UIState {
    camera_pos: Vec3<f32>,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            camera_pos: Vec3::zero(),
        }
    }
}
impl EguiInstance {
    pub fn new(window: &winit::window::Window) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: window.inner_size().width,
            physical_height: window.inner_size().height,
            scale_factor: window.scale_factor(),
            font_definitions: egui::FontDefinitions::default(),
            style: Default::default(),
        });
        Self {
            platform,
            state: Default::default(),
        }
    }
}

pub fn draw_camera_settings(platform: &mut Platform, camera: &mut Camera) {
    egui::Window::new("Camera Settings")
        .default_size([200.0, 200.0])
        .show(&platform.context(), |ui| {
            ui.label("Position [X Y Z]");
            Grid::new("camera_position").show(ui, |ui| {
                ui.add(DragValue::new(&mut camera.pos.x));
                ui.add(DragValue::new(&mut camera.pos.y));
                ui.add(DragValue::new(&mut camera.pos.z));
                ui.end_row();
            });
            ui.separator();

            ui.label("Target [X Y Z]");
            Grid::new("camera_target").show(ui, |ui| {
                ui.add(DragValue::new(&mut camera.target.x));
                ui.add(DragValue::new(&mut camera.target.y));
                ui.add(DragValue::new(&mut camera.target.z));
                ui.end_row();
            });
            ui.separator();
            ui.label("UP Vector [X Y Z]");
            Grid::new("camera_up_vector").show(ui, |ui| {
                ui.add(DragValue::new(&mut camera.up.x));
                ui.add(DragValue::new(&mut camera.up.y));
                ui.add(DragValue::new(&mut camera.up.z));
                ui.end_row();
            });
            ui.separator();
            ui.label("Vertical FOV in degrees");
            ui.add(DragValue::new(&mut camera.fov_y_deg));
            ui.separator();
            ui.label("Aspect Ratio [W,H]");
            Grid::new("aspect_ratio").show(ui, |ui| {
                ui.add(DragValue::new(&mut camera.width));
                ui.add(DragValue::new(&mut camera.height));
                ui.end_row();
            });
            ui.separator();
            ui.label("Near and Far Z-Planes [N,F]");
            Grid::new("z-planes").show(ui, |ui| {
                ui.add(DragValue::new(&mut camera.near_plane));
                ui.add(DragValue::new(&mut camera.far_plane));
                ui.end_row();
            });
        });
}
