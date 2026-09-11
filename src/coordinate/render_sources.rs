#[derive(Clone, Copy)]
pub(crate) struct MyRenderResources {
    pub target_width: u32,
    pub target_height: u32,
    pub(crate) s: f32,
    pub(crate) center: [f32; 2],
}
