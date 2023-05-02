use ui::EguiInstance;
use winit::{
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
        .build(&event_loop)
        .expect("Failed to create window");

    let mut renderer = pollster::block_on(renderer::Renderer::new(&window));
    let mut gui = EguiInstance::new(&window);
    event_loop.run(move |generic_event, _, control_flow| {
        gui.platform.handle_event(&generic_event);

        match generic_event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !renderer.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(size) => {
                            renderer.resize(*size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            renderer.resize(**new_inner_size);
                        }
                        _ => {}
                    };
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                renderer.update();
                match renderer.render(&window, &mut gui.platform) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
