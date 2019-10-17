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

mod finalize {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shaders/finalize.comp"
    }
}
pub use finalize::Layout as FinalizeShaderLayout;
pub use finalize::Shader as FinalizeShader;

mod randomize {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shaders/randomize.comp"
    }
}
pub use randomize::Layout as RandomizeShaderLayout;
pub use randomize::Shader as RandomizeShader;

mod screen_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/screen.vert"
    }
}
pub use screen_fs::Layout as ScreenFragmentShaderLayout;
pub use screen_fs::Shader as ScreenFragmentShader;

mod screen_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/screen.frag"
    }
}
pub use screen_vs::Layout as ScreenVertexShaderLayout;
pub use screen_vs::Shader as ScreenVertexShader;

mod simulate {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shaders/simulate.comp"
    }
}
pub use simulate::Layout as SimulateShaderLayout;
pub use simulate::Shader as SimulateShader;

pub fn load_finalize_shader(device: Arc<Device>) -> FinalizeShader {
    finalize::Shader::load(device).unwrap()
}

pub fn load_randomize_shader(device: Arc<Device>) -> RandomizeShader {
    randomize::Shader::load(device).unwrap()
}

pub fn load_screen_vertex_shader(device: Arc<Device>) -> ScreenVertexShader {
    screen_vs::Shader::load(device).unwrap()
}

pub fn load_screen_fragment_shader(device: Arc<Device>) -> ScreenFragmentShader {
    screen_fs::Shader::load(device).unwrap()
}

pub fn load_simulate_shader(device: Arc<Device>) -> SimulateShader {
    simulate::Shader::load(device).unwrap()
}
