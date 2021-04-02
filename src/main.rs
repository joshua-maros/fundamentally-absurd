mod app;
mod dispatch_manager;
mod init;
mod options;
mod presenter;
mod renderer;
mod shaders;

fn main() {
    app::App::new().start();
}
