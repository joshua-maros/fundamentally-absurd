use winit::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

mod app;
mod dispatch_manager;
mod init;
mod presenter;
mod renderer;
mod shaders;

use dispatch_manager::DispatchManager;
use presenter::Presenter;
use renderer::Renderer;

fn main() {
    let mut app = app::App::new();
    app.start();
}
