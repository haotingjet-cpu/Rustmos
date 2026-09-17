const LOAD: u32 = 0x00000000;
const ADD:  u32 = 0x00000001;
const SUB:  u32 = 0x00000002;
const MUL:  u32 = 0x00000003;
const DIV:  u32 = 0x00000004;
const LOADX:u32 = 0x00000005;

fn unpack_u32_to_4u8(i: u32) -> vec4<u32> {
    let first = (i >> 0u ) & 0xffu;
    let second = (i >> 8u ) & 0xffu;
    let third = (i >> 16u ) & 0xffu;
    let last = (i >> 24u ) & 0xffu;
    return vec4<u32>(first, second, third, last);
}

struct InputInstruction {
    instruction_number: u32,
    instruction: array<vec4<u32>, 64>, // 256 組指令 (LOAD, LOADX, ADD ...)
    para: array<vec4<f32>, 64>, // 256 個參數
}

struct Vertex {
    position: vec4<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> input: InputInstruction;
@group(0) @binding(1) var<storage, read_write> output_vertices: array<Vertex>;

@compute @workgroup_size(64)
fn vm_main()
{
    var register = array<vec4<f32>, 64>;
    for (var: i = 0; i < input.instruction_number; i = i + 1)
    {
        let array_idx = i / 4;
        let vec_idx = i % 4;

        let code = input.instruction[array_idx][vec_idx];

        let codes = unpack_u32_to_4u8(code);
        switch codes[0] {
            case LOAD: {
                let idx = codes[1];
                register[idx / 4][idx % 4] = input.para[codes[2]]
            }

            case ADD: {
                let idx = codes[1];
                let first_idx = codes[2];
                let second_idx = codes[3];
                register[idx / 4][idx % 4] = register[first_idx / 4][first_idx % 4] + register[second_idx / 4][second_idx % 4];
            }
            case SUB: {
                let idx = codes[1];
                let first_idx = codes[2];
                let second_idx = codes[3];
                register[idx / 4][idx % 4] = register[first_idx / 4][first_idx % 4] - register[second_idx / 4][second_idx % 4];
            }
        }
    }
}
