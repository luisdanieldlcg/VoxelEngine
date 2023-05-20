use crate::renderer::Renderer;

pub struct Window {
    winit_impl: winit::window::Window,
}

impl Window {
    pub fn new() -> (Self, Renderer, winit::event_loop::EventLoop<()>) {
        let event_loop = winit::event_loop::EventLoop::new();
        let winit_impl = winit::window::WindowBuilder::new()
            .with_title("VoxelEngine")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap();

        let renderer = pollster::block_on(Renderer::new(&winit_impl));

        let this = Self { winit_impl };
        (this, renderer, event_loop)
    }

    pub fn grab_cursor(&mut self, grab: bool) {
        self.winit_impl.set_cursor_visible(!grab);
        // Do not crash if this fails
        let _ = self
            .winit_impl
            .set_cursor_grab(winit::window::CursorGrabMode::Locked);
    }
    pub fn scale_factor(&self) -> f32 {
        self.winit_impl.scale_factor() as f32
    }
}
