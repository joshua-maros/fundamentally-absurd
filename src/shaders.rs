use vulkano::device::Device;

use std::sync::Arc;

// Unfortunately the shader! macro does not trigger a recompile whenever source code changes.
fn _watchdog() {
    let _source = include_bytes!("../shaders/finalize.comp");
    let _source = include_bytes!("../shaders/randomize.comp");
    let _source = include_bytes!("../shaders/screen.vert");
    let _source = include_bytes!("../shaders/screen.frag");
    let _source = include_bytes!("../shaders/simulate.comp");
}

pub mod finalize {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shaders/finalize.comp"
    }
}

pub mod randomize {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shaders/randomize.comp"
    }
}

pub mod screen_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/screen.vert"
    }
}

pub mod screen_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/screen.frag"
    }
}

pub mod simulate {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shaders/simulate.comp"
    }
}

pub fn load_finalize_shader(device: Arc<Device>) -> finalize::Shader {
    finalize::Shader::load(device).unwrap()
}

pub fn load_randomize_shader(device: Arc<Device>) -> randomize::Shader {
    randomize::Shader::load(device).unwrap()
}

pub fn load_screen_vertex_shader(device: Arc<Device>) -> screen_vs::Shader {
    screen_vs::Shader::load(device).unwrap()
}

pub fn load_screen_fragment_shader(device: Arc<Device>) -> screen_fs::Shader {
    screen_fs::Shader::load(device).unwrap()
}

pub fn load_simulate_shader(device: Arc<Device>) -> simulate::Shader {
    simulate::Shader::load(device).unwrap()
}
