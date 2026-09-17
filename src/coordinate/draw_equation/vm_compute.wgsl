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
    _pad0: u32, _pad1: u32, _pad2: u32,
    color: vec4<f32>,
    instruction: array<vec4<u32>, 64>, // 256 組指令 (LOAD, LOADX, ADD ...)
    para: array<vec4<f32>, 64>, // 256 個參數
}

struct Vertex {
    position: vec4<f32>,
    color: vec4<f32>,
}

struct Coordinate {
    max_x: f32,
    min_x: f32,
    _max_y: f32,
    _min_y: f32
}

@group(0) @binding(0) var<uniform> coordinate: Coordinate;
@group(0) @binding(1) var<uniform> input: InputInstruction;
@group(0) @binding(2) var<storage, read_write> output_vertices: array<Vertex>;

@compute @workgroup_size(64)
fn vm_main(@builtin(global_invocation_id) id: vec3<u32>)
{
    let thread_id = id.x;

    if (thread_id >= arrayLength(&output_vertices)) {
        return;
    }

    // rust 開了 N 個線程
    let n = 10u;
    let total_threads = n * 64u;
    let target_x = (coordinate.min_x * f32(total_threads - thread_id) + coordinate.max_x * f32(thread_id)) / f32(total_threads);

    var register = array<vec4<f32>, 64>();
    for (var i = 0u; i < input.instruction_number; i = i + 1u)
    {
        let array_idx = i / 4u;
        let vec_idx = i % 4u;

        let code = input.instruction[array_idx][vec_idx];

        let codes = unpack_u32_to_4u8(code);
        switch codes[0u] {
            case LOAD: {
                let idx = codes[1u];
                register[idx / 4u][idx % 4u] = input.para[codes[2u]];
            }
            case LOADX: {
                let idx = codes[1u];

                register[idx / 4u][idx % 4u] = target_x;
            }
            case ADD: {
                let idx = codes[1u];
                let first_idx = codes[2u];
                let second_idx = codes[3u];
                register[idx / 4u][idx % 4u] = register[first_idx / 4u][first_idx % 4u] + register[second_idx / 4u][second_idx % 4u];
            }
            case SUB: {
                let idx = codes[1u];
                let first_idx = codes[2u];
                let second_idx = codes[3u];
                register[idx / 4u][idx % 4u] = register[first_idx / 4u][first_idx % 4u] - register[second_idx / 4u][second_idx % 4u];
            }
            case MUL: {
                let idx = codes[1u];
                let first_idx = codes[2u];
                let second_idx = codes[3u];
                register[idx / 4u][idx % 4u] = register[first_idx / 4u][first_idx % 4u] * register[second_idx / 4u][second_idx % 4u];
            }
            case DIV: {
                let idx = codes[1u];
                let first_idx = codes[2u];
                let second_idx = codes[3u];
                register[idx / 4u][idx % 4u] = register[first_idx / 4u][first_idx % 4u] / register[second_idx / 4u][second_idx % 4u];
            }

            default: {}
        }
    }

    output_vertices[thread_id].position = vec4<f32>(target_x,register[0u][0u], 0.0, 1.0);
    output_vertices[thread_id].color = input.color;
}
