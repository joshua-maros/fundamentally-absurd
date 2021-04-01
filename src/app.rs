use crate::{dispatch_manager::DispatchManager, init, presenter::Presenter, renderer::Renderer};
use std::sync::Arc;
use winit::{ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct App {
    dispatcher: DispatchManager,
    renderer: Renderer,
    events_loop: EventsLoop,
    coefficients: Vec<i16>,
}

impl App {
    pub fn new() -> Self {
        let init::InitResult {
            device,
            queue,
            surface,
            events_loop,
            swapchain,
            swapchain_images,
        } = init::init();

        let presenter = Arc::new(Presenter::new(
            device.clone(),
            queue.clone(),
            (512, 512),
            swapchain.format(),
        ));

        let mut renderer = Renderer::new(
            device.clone(),
            queue.clone(),
            presenter.get_presented_image(),
        );
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);
        let coefficients: Vec<_> = args.iter().map(|arg| arg.parse().unwrap()).collect();
        renderer.set_parameters(&coefficients[..]);

        let dispatcher = DispatchManager::new(
            device,
            queue,
            Arc::clone(&presenter),
            Arc::clone(&surface),
            swapchain.clone(),
            &swapchain_images,
        );

        Self {
            dispatcher,
            renderer,
            events_loop,
            coefficients,
        }
    }

    pub fn start(&mut self) {
        let Self {
            dispatcher,
            renderer,
            coefficients,
            events_loop,
        } = self;
        loop {
            let success = dispatcher
                .create_and_submit_commands(|builder| renderer.add_render_commands(builder));
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
                    VirtualKeyCode::F => {
                        renderer.skip_frames(1);
                        renderer.pause();
                    }
                    VirtualKeyCode::Space => {
                        let divisor = coefficients[0];
                        coefficients[divisor as usize] += 1;
                        for index in (1..divisor as usize).rev() {
                            if coefficients[index + 1] >= divisor {
                                coefficients[index + 1] = 0;
                                coefficients[index] += 1;
                            }
                        }
                        for parameter in &*coefficients {
                            print!("{} ", parameter);
                        }
                        println!("");
                        renderer.set_parameters(&coefficients);
                        renderer.reset_world();
                    }
                    VirtualKeyCode::Back => {
                        let divisor = coefficients[0];
                        coefficients[divisor as usize] -= 1;
                        for index in (1..divisor as usize).rev() {
                            if coefficients[index + 1] == -1 {
                                coefficients[index + 1] = divisor - 1;
                                coefficients[index] -= 1;
                            }
                        }
                        for parameter in &*coefficients {
                            print!("{} ", parameter);
                        }
                        println!("");
                        renderer.set_parameters(&coefficients);
                        renderer.reset_world();
                    }
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
        }
    }
}
