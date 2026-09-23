pub mod ast;
pub mod vm_renderer;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ByteCode {
    pub data: [f32; 256],
    pub code: [u32; 64],
}

pub fn equation_to_ast(equa: &String) {
    let equa_iter = equa.chars();

    enum State {
        FindY,
        FindEqual,
        Anysis,
    }

    let mut now = State::FindY;
    for ch in equa_iter {
        if ch == ' ' {
            continue;
        }
        match now {
            State::FindY => {
                if ch == 'y' {
                    now = State::FindEqual;
                }
            }
            State::FindEqual => {
                if ch == '=' {
                    now = State::Anysis
                }
            }
            State::Anysis => {}
        }
    }
}
