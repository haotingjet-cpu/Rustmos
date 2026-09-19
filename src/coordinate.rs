pub(crate) mod call_back;
pub(crate) mod draw_wquation;
pub(crate) mod offscreen_renderer;
pub(crate) mod transform;

/// transform: width / height
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CoordinateDiscripter {
    pub(crate) min_x: f32,
    pub(crate) max_x: f32,
    pub(crate) min_y: f32,
    pub(crate) max_y: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Transform {
    pub(crate) transform: f32,
    pub(crate) _pad: [u32; 3],
}
