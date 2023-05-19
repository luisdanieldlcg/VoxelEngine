use crate::{engine::VoxelEngine, window::Window};

pub fn init() {
    let (window, mut renderer, event_loop) = Window::new();

    event_loop.run(move |event, _, flow| match event {
        winit::event::Event::MainEventsCleared => if let Ok(_) = renderer.render() {},
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *flow = winit::event_loop::ControlFlow::Exit
            }
            _ => (),
        },
        _ => (),
    });
}
