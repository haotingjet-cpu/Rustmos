#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

impl Vertex {
    pub fn init() -> [Self; crate::pub_const::VERTEX_NUM] {
        [Self {
            position: [0.0; 4],
            color: crate::ui::Color::Blue.get_raw(),
        }; crate::pub_const::VERTEX_NUM]
    }
}
