pub(crate) mod call_back;
pub(crate) mod draw_wquation;
pub(crate) mod offscreen_renderer;
pub(crate) mod render_data;

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
