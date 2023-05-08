use std::time::Instant;

use ui::EguiInstance;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod renderer;
pub mod ui;
fn main() {
    run();
}

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(1000.0, 800.0))
        .build(&event_loop)
        .expect("Failed to create window");
    window.set_cursor_visible(false);

    window
        .set_cursor_grab(winit::window::CursorGrabMode::Locked)
        .unwrap();

    let gui = EguiInstance::new(&window);
    let mut renderer = pollster::block_on(renderer::Renderer::new(&window, gui));
    let mut last_render_time = Instant::now();

    let mut locked_input = false;

    event_loop.run(move |generic_event, _, control_flow| {
        renderer.gui.platform.handle_event(&generic_event);
        *control_flow = ControlFlow::Poll;
        match generic_event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        // update locked_input
                        // but only if the key is pressed down
                        if input.state == ElementState::Pressed {
                            if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                                locked_input = !locked_input;

                                window
                                    .set_cursor_grab(if locked_input {
                                        window.set_cursor_visible(true);
                                        winit::window::CursorGrabMode::None
                                    } else {
                                        window.set_cursor_visible(false);
                                        winit::window::CursorGrabMode::Locked
                                    })
                                    .unwrap();
                            }
                        }
                        if locked_input {
                            return;
                        }
                        renderer.on_key_pressed(input);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        renderer.resize(*size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.resize(**new_inner_size);
                    }
                    // WindowEvent::CursorMoved { position, .. } => {
                    //     renderer.on_cursor_moved((position.x as f32, position.y as f32));
                    // }
                    _ => {}
                };
            }
            Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if locked_input {
                    return;
                }
                renderer.on_mouse_motion(delta);
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                renderer.update(last_render_time.elapsed());
                last_render_time = Instant::now();
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
