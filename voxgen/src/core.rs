use crate::{engine::VoxelEngine, window::Window};
use std::time::Instant;

pub fn init() {
    let (window, mut renderer, event_loop) = Window::new();
    let mut engine = VoxelEngine {
        renderer,
        window,
        locked_input: false,
    };
    let mut last_render_time = Instant::now();

    event_loop.run(move |event, _, flow| {
        engine.renderer_mut().gui.platform.handle_event(&event);
        if !engine.locked_input {
            engine.renderer_mut().input(&event);
        }

        match event {
            winit::event::Event::MainEventsCleared => {
                let scale_factor = engine.window.scale_factor();
                let dt = last_render_time.elapsed();
                engine.renderer_mut().update(dt);
                last_render_time = Instant::now();
                match engine.renderer_mut().render(scale_factor, dt.as_secs_f32()) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        engine.on_key_pressed(input.virtual_keycode);
                    }
                }
                winit::event::WindowEvent::CloseRequested => {
                    *flow = winit::event_loop::ControlFlow::Exit
                }
                winit::event::WindowEvent::Resized(size) => {
                    engine.renderer_mut().resize(size);
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    engine.renderer_mut().resize(*new_inner_size);
                }
                _ => (),
            },
            _ => (),
        }
    });
}
