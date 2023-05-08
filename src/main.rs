use std::time::Instant;

use ui::EguiInstance;
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;
mod ui;
fn main() {
    run();
}

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)
        .expect("Failed to create window");

    let gui = EguiInstance::new(&window);
    let mut renderer = pollster::block_on(renderer::Renderer::new(&window, gui));
    let mut render_delta = Instant::now();
    event_loop.run(move |generic_event, _, control_flow| {
        renderer.gui.platform.handle_event(&generic_event);
        *control_flow = ControlFlow::Poll;
        match generic_event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => renderer.on_key_pressed(input),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        renderer.resize(*size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.resize(**new_inner_size);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        renderer.on_cursor_moved((position.x as f32, position.y as f32));
                    }
                    _ => {}
                };
            }
            Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion { delta },
                ..
            } => {
                renderer.on_mouse_motion(delta);
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = Instant::now();
                let dt = now - render_delta;
                render_delta = now;
                renderer.update(dt);
                match renderer.render(&window) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}
