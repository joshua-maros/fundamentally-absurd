pub const WORLD_SIZE: u32 = 1024;
pub const PARAMETER_SPACE: usize = 128;

#[derive(Clone)]
pub struct Options {
    pub kernel_arguments: [i16; PARAMETER_SPACE],
    pub offset: [i32; 2],
    pub zoom: u32,
    pub rate: u32,
    pub skip: u32,
    pub reset: bool,
}

impl Default for Options {

    fn default() -> Self {
        Self {
            kernel_arguments: [0; PARAMETER_SPACE],
            offset: [0, 0],
            zoom: 1,
            rate: 1,
            skip: 0,
            reset: true,
        }
    }
}