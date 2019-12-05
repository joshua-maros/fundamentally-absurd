use winit::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

mod dispatch_manager;
mod init;
mod presenter;
mod renderer;
mod shaders;

use dispatch_manager::DispatchManager;
use presenter::Presenter;
use renderer::Renderer;

fn main() {
    let init::InitResult {
        device,
        queue,
        surface,
        mut events_loop,
        swapchain,
        swapchain_images,
    } = init::init();
    let window = surface.window();

    let presenter = Presenter::new(
        device.clone(),
        queue.clone(),
        (512, 512),
        swapchain.format(),
    );

    let mut renderer = Renderer::new(
        device.clone(),
        queue.clone(),
        presenter.get_presented_image(),
    );
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    renderer.set_parameters(&args.iter().map(|arg| arg.parse().unwrap()).collect());

    let mut dispatcher = DispatchManager::new(
        device, 
        queue, 
        &presenter, 
        window, 
        swapchain.clone(), 
        &swapchain_images
    );

    let mut total_frames = 0;
    let mut total_frame_time = 0;
    loop {
        let frame_start = std::time::Instant::now();
        let success = dispatcher.create_and_submit_commands(|builder| renderer.add_render_commands(builder));
        if !success {
            continue;
        }

        let mut done = false;
        events_loop.poll_events(|ev| match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => done = true,
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                renderer.set_offset(position.x as f32 / 512.0, 1.0 - position.y as f32 / 512.0);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(code),
                                ..
                            },
                        ..
                    },
                ..
            } => match code {
                VirtualKeyCode::Escape => done = true,
                VirtualKeyCode::Equals => renderer.offset_zoom(true),
                VirtualKeyCode::Subtract => renderer.offset_zoom(false),
                VirtualKeyCode::Comma => renderer.offset_rate(false),
                VirtualKeyCode::Period => renderer.offset_rate(true),
                VirtualKeyCode::R => renderer.reset_world(),
                _ => (),
            },
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(code),
                                ..
                            },
                        ..
                    },
                ..
            } => match code {
                _ => (),
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => dispatcher.invalidate_swapchain(),
            _ => (),
        });
        if done {
            return;
        }

        total_frame_time += frame_start.elapsed().as_millis();
        total_frames += 1;
    }
}
