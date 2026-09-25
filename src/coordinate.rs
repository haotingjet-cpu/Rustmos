pub(crate) mod call_back;
pub(crate) mod draw_wquation;
pub(crate) mod offscreen_renderer;
pub(crate) mod render_data;
pub(crate) mod vertex;

/// transform: width / height
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CoordinateUniform {
    pub(crate) transform: f32,
    pub(crate) s: f32,
    pub(crate) center: [f32; 2],

    // first 16 byte
    pub(crate) size: [f32; 2],
    pub(crate) _pad: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CoordinateUniformForCompute {
    pub max_x: f32,
    pub min_x: f32,
    _pad: [u32; 2],
}

impl CoordinateUniformForCompute {
    pub fn init() -> Self {
        Self {
            max_x: 2.0,
            min_x: -2.0,
            _pad: [0; 2],
        }
    }
}
