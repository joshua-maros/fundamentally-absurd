use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    dispatch_manager::DispatchManager,
    init,
    options::{Options, PARAMETER_SPACE, WORLD_SIZE},
    presenter::Presenter,
    renderer::Renderer,
    stats::{AutomaticJudgement, Judge, Scorer, Stats},
};
use std::sync::Arc;

struct AppData {
    options: Options,
    renderer: Renderer,
    dispatcher: DispatchManager,
}

pub struct App {
    events_loop: EventLoop<()>,
    data: AppData,
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
            (1024, 1024),
            swapchain.format(),
        ));

        let renderer = Renderer::new(
            device.clone(),
            queue.clone(),
            presenter.get_presented_image(),
        );
        let mut kernel_arguments = [0i16; PARAMETER_SPACE];
        kernel_arguments[0] = 2;
        for (index, value) in std::env::args().skip(1).enumerate() {
            kernel_arguments[index] = value.parse().unwrap();
        }

        let dispatcher = DispatchManager::new(
            device,
            queue,
            Arc::clone(&presenter),
            Arc::clone(&surface),
            swapchain.clone(),
            &swapchain_images,
        );

        Self {
            events_loop,
            data: AppData {
                options: Options {
                    kernel_arguments,
                    ..Default::default()
                },
                renderer,
                dispatcher,
            },
        }
    }

    pub fn start(self) -> ! {
        let Self {
            mut data,
            events_loop,
            ..
        } = self;

        events_loop.run(move |ev, _, flow| match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                data.set_offset(position.x as f32 / 1024.0, 1.0 - position.y as f32 / 1024.0);
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
                VirtualKeyCode::Escape => *flow = ControlFlow::Exit,
                code => data.on_key(code),
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
            } => data.dispatcher.invalidate_swapchain(),
            Event::MainEventsCleared => {
                let success = data.render();
                data.after_frame();
                if !success {
                    return;
                }
            }
            _ => (),
        });
    }
}

impl AppData {
    fn render(&mut self) -> bool {
        self.renderer.render(&mut self.dispatcher, &self.options)
    }

    fn set_offset(&mut self, x: f32, y: f32) {
        self.options.offset[0] = (x * WORLD_SIZE as f32) as i32;
        self.options.offset[1] = (y * WORLD_SIZE as f32) as i32;
    }

    fn offset_zoom(&mut self, increment: bool) {
        if increment {
            self.options.zoom += 1;
        } else {
            self.options.zoom -= 1;
        }
        if self.options.zoom > 4 {
            self.options.zoom = 4;
        } else if self.options.zoom < 1 {
            self.options.zoom = 1;
        }
        println!("{}x zoom", self.options.zoom);
    }

    fn offset_rate(&mut self, increase: bool) {
        if increase {
            if self.options.rate == 0 {
                self.options.rate = 1;
            } else {
                self.options.rate *= 2;
            }
        } else {
            if self.options.rate > 1 {
                self.options.rate /= 2;
            } else {
                self.options.rate = 0;
            }
        }
        println!("{} generations per frame", self.options.rate);
    }

    fn pause(&mut self) {
        self.options.rate = 0;
    }

    fn skip_frames(&mut self, num_frames: u32) {
        self.options.skip += num_frames;
    }

    fn reset_world(&mut self) {
        self.options.reset = true;
        self.skip_frames(1);
    }

    fn offset_arguments(&mut self, increase: bool) {
        let divisor = self.options.kernel_arguments[0];
        if increase {
            self.options.kernel_arguments[divisor as usize] += 1;
        } else {
            self.options.kernel_arguments[divisor as usize] -= 1;
        }
        for index in (1..divisor as usize).rev() {
            if self.options.kernel_arguments[index + 1] >= divisor {
                self.options.kernel_arguments[index + 1] = 0;
                self.options.kernel_arguments[index] += 1;
            } else if self.options.kernel_arguments[index + 1] == -1 {
                self.options.kernel_arguments[index + 1] = divisor - 1;
                self.options.kernel_arguments[index] -= 1;
            }
        }
        self.reset_world();
    }

    fn compute_judgement(&mut self) -> AutomaticJudgement {
        let test_options = Options {
            reset: true,
            rate: 0,
            skip: 1,
            display: false,
            ..self.options.clone()
        };
        self.renderer.render(&mut self.dispatcher, &test_options);
        let mut judge = Judge::new(Stats::of(&self.renderer));
        let test_options = Options {
            reset: false,
            skip: 20,
            ..test_options
        };
        for _ in 0..4 {
            self.renderer.render(&mut self.dispatcher, &test_options);
            judge.push_snapshot(Stats::of(&self.renderer));
            let judgement = judge.judgement();
            if !judgement.is_unknown() {
                println!("{:?}", judgement);
                return judgement;
            }
        }
        AutomaticJudgement::Unknown
    }

    fn compute_score(&mut self) -> f32 {
        let test_options = Options {
            reset: false,
            rate: 0,
            skip: 1,
            display: false,
            ..self.options.clone()
        };
        let mut scorer = Scorer::new();
        for _ in 0..100 {
            self.renderer.render(&mut self.dispatcher, &test_options);
            scorer.add_snapshot(&self.renderer);
        }
        let densities = scorer.find_pattern_densities();
        let score = scorer.compute_score(&densities[..]);
        let params = &self.options.kernel_arguments[..];
        let last_zero = params.len() - params.iter().rev().position(|a| *a != 0).unwrap();
        let params = (&params[..last_zero])
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("-");
        let filename = format!("captures/8/SCORE {:08.2} PARAMS {}.gif", score, params);
        println!("{}", filename);
        scorer.create_gif(&densities[..], &filename[..]);
        score
    }

    fn skip_uninteresting(&mut self) {
        self.offset_arguments(true);
        while !self.compute_judgement().is_interesting() {
            self.offset_arguments(true);
        }
        let divisor = self.options.kernel_arguments[0];
        for argument in &self.options.kernel_arguments[0..1 + divisor as usize] {
            print!("{} ", argument);
        }
        println!("");
        println!("{:?}", self.compute_score());
    }

    fn on_key(&mut self, code: VirtualKeyCode) {
        match code {
            VirtualKeyCode::Equals => self.offset_zoom(true),
            VirtualKeyCode::Minus => self.offset_zoom(false),
            VirtualKeyCode::Comma => self.offset_rate(false),
            VirtualKeyCode::Period => self.offset_rate(true),
            VirtualKeyCode::R => self.reset_world(),
            VirtualKeyCode::F => {
                self.skip_frames(1);
                self.pause();
            }
            VirtualKeyCode::Space => self.skip_uninteresting(),
            VirtualKeyCode::Return => {
                while self.options.kernel_arguments[1] == 0 {
                    self.skip_uninteresting()
                }
            }
            VirtualKeyCode::Back => self.offset_arguments(false),
            _ => (),
        }
    }

    fn after_frame(&mut self) {
        self.options.reset = false;
        self.options.skip = 0;
    }
}
