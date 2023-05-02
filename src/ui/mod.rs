use egui_winit_platform::{Platform, PlatformDescriptor};

pub struct EguiInstance {
    pub platform: Platform,
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
        Self { platform }
    }
}
