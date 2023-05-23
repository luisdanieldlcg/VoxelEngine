use egui_winit_platform::{Platform, PlatformDescriptor};
use vek::Vec3;

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

pub fn draw_debugging_settings(platform: &mut Platform, dt: f32, wireframe: &mut bool) {
    egui::Window::new("Debug Settings")
        .default_size([200.0, 200.0])
        .show(&platform.context(), |ui| {
            ui.label("[FPS]: ".to_owned() + &(1.0 / dt).to_string());
            ui.checkbox(wireframe, "Toggle Wireframe mode");
        });
}

pub fn draw_camera_settings(
    platform: &mut Platform,
    // controller: &mut CameraController,
    pos: Vec3<f32>,
) {
    egui::Window::new("Camera Settings")
        .default_size([200.0, 200.0])
        .collapsible(true)
        .title_bar(true)
        .show(&platform.context(), |ui| {
            // ui.label("Speed");
            // ui.add(DragValue::new(&mut controller.speed));
            // ui.separator();

            // ui.label("Sensitivity");
            // ui.add(DragValue::new(&mut controller.sensitivity));
            // ui.separator();

            ui.label("Position [X Y Z]");
            ui.label(format!("[{:.2} {:.2} {:.2}]", pos.x, pos.y, pos.z,));
        });
}
