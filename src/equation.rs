pub mod ast;
pub mod vm_renderer;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InputInstruction {
    pub instruction_len: u32,
    pub _pad: [u32; 3],
    pub color: [f32; 4],
    pub instruction: [u32; 64],
    pub para: [f32; 256],
}

impl InputInstruction {
    pub fn init() -> Self {
        Self {
            instruction_len: 0,
            _pad: [0; 3],
            color: crate::ui::Color::Blue.get_raw(),
            instruction: [0; 64],
            para: [0.0; 256],
        }
    }
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
