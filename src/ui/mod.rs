use egui::{DragValue, Grid};
use egui_winit_platform::{Platform, PlatformDescriptor};
use vek::Vec3;

pub struct EguiInstance {
    pub platform: Platform,
    pub (super) state: UIState,
}

pub struct UIState {
    camera_pos: Vec3<f32>,
}

impl Default for UIState {
    fn default() -> Self {
        Self { camera_pos: Vec3::zero() }
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

pub fn draw_camera_settings(platform: &mut Platform, state: &mut UIState) {
    egui::Window::new("Camera Settings")
        .default_size([200.0, 200.0])
        .show(&platform.context(), |ui| {
            ui.label("Position X Y Z");
            Grid::new("camera_position").show(ui, |ui| {
                ui.add(DragValue::new(&mut state.camera_pos.x));
                ui.add(DragValue::new(&mut state.camera_pos.y));
                ui.add(DragValue::new(&mut state.camera_pos.z));
                ui.end_row();
            })
        });
}