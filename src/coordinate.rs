pub(crate) mod call_back;
pub(crate) mod offscreen_renderer;

/// transform: width / height
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CoordinateUniform {
    pub(crate) transform: f32,
    pub(crate) s: f32,
    pub(crate) center: [f32; 2],
}
