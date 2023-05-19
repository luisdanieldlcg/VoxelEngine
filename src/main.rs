use std::time::Instant;

use global::GlobalState;
use ui::EguiInstance;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod block;
pub mod direction;
pub mod global;
pub mod renderer;
pub mod scene;
pub mod ui;
pub mod world;

fn main() {
    run();
}

pub fn run() {
    env_logger::init();
    voxgen::core::init();

    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new()
    //     .with_inner_size(winit::dpi::LogicalSize::new(1000.0, 800.0))
    //     .build(&event_loop)
    //     .expect("Failed to create window");
    // window.set_cursor_visible(false);

    // window
    //     .set_cursor_grab(winit::window::CursorGrabMode::Locked)
    //     .unwrap_or(()); // do not crash

    // let gui = EguiInstance::new(&window);
    // let renderer = pollster::block_on(renderer::Renderer::new(&window, gui));
    // let mut last_render_time = Instant::now();

    // let mut state = GlobalState::new(renderer);

    // event_loop.run(move |generic_event, _, control_flow| {
    //     *control_flow = ControlFlow::Poll;
    //     state.renderer.gui.platform.handle_event(&generic_event);
    //     if !state.locked_input {
    //         state.renderer.input(&generic_event);
    //     }
    //     match generic_event {
    //         Event::WindowEvent {
    //             ref event,
    //             window_id,
    //         } if window_id == window.id() => {
    //             match event {
    //                 WindowEvent::KeyboardInput { input, .. } => {
    //                     // update locked_input
    //                     // but only if the key is pressed down
    //                     if input.state == ElementState::Pressed {
    //                         if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
    //                             state.locked_input = !state.locked_input;

    //                             window
    //                                 .set_cursor_grab(if state.locked_input {
    //                                     window.set_cursor_visible(true);
    //                                     winit::window::CursorGrabMode::None
    //                                 } else {
    //                                     window.set_cursor_visible(false);
    //                                     winit::window::CursorGrabMode::Locked
    //                                 })
    //                                 .unwrap_or(());
    //                         }
    //                     }
    //                     if input.state == ElementState::Pressed {
    //                         if input.virtual_keycode == Some(VirtualKeyCode::F12) {
    //                             state.renderer.world_renderer.wireframe =
    //                                 !state.renderer.world_renderer.wireframe;
    //                         }
    //                     }
    //                 }
    //                 WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
    //                 WindowEvent::Resized(size) => {
    //                     state.renderer.resize(*size);
    //                 }
    //                 WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
    //                     state.renderer.resize(**new_inner_size);
    //                 }
    //                 _ => {}
    //             };
    //         }
    //         Event::RedrawRequested(window_id) if window_id == window.id() => {
    //             let dt = last_render_time.elapsed();
    //             state.renderer.update(dt);
    //             last_render_time = Instant::now();
    //             match state.renderer.render(&window, dt.as_secs_f32()) {
    //                 Ok(_) => {}
    //                 Err(e) => eprintln!("{:?}", e),
    //             }
    //         }
    //         Event::MainEventsCleared => window.request_redraw(),
    //         _ => {}
    //     }
    // });
}
