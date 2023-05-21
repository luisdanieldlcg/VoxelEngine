use crate::renderer::Renderer;

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "VoxelEngine".to_string(),
            size: (1024, 768),
        }
    }
}

pub struct Window {
    winit_impl: winit::window::Window,
}

impl Window {
    pub fn new(settings: WindowSettings) -> (Self, Renderer, winit::event_loop::EventLoop<()>) {
        let event_loop = winit::event_loop::EventLoop::new();
        let winit_impl = winit::window::WindowBuilder::new()
            .with_title(settings.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                settings.size.0,
                settings.size.1,
            ))
            .build(&event_loop)
            .unwrap();

        let renderer = pollster::block_on(Renderer::new(&winit_impl));

        let this = Self { winit_impl };
        (this, renderer, event_loop)
    }

    pub fn grab_cursor(&mut self, grab: bool) {
        self.winit_impl.set_cursor_visible(!grab);
        // Not unwrapping to avoid panicking
        if grab {
            let _ = self
                .winit_impl
                .set_cursor_grab(winit::window::CursorGrabMode::Locked);
        } else {
            let _ = self
                .winit_impl
                .set_cursor_grab(winit::window::CursorGrabMode::None);
        }
    }
    pub fn scale_factor(&self) -> f32 {
        self.winit_impl.scale_factor() as f32
    }
}
