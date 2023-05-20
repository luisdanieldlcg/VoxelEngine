use std::time::Instant;
use crate::window::Window;

pub fn init() {
    let (window, mut renderer, event_loop) = Window::new();
    let mut last_render_time = Instant::now();

    event_loop.run(move |event, _, flow|{
        renderer.input(&event);

        match event {
            winit::event::Event::MainEventsCleared => {
                let scale_factor = window.scale_factor();
                let dt = last_render_time.elapsed();
                renderer.update(dt);
                last_render_time = Instant::now();
                renderer.render(scale_factor, dt.as_secs_f32()).unwrap();
            },
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *flow = winit::event_loop::ControlFlow::Exit
                }
                winit::event::WindowEvent::Resized(size) => {
                    renderer.resize(size);
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    renderer.resize(*new_inner_size);
                }
                _ => (),
            },
            _ => (),
        }
    });
}
