// vertex.wgsl
const WIDTH: f32 = 0.003;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct Coordinate {
    transform: f32,
    s: f32,
    center: vec2<f32>,
    width: f32,
    height: f32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<uniform> coordinate: Coordinate;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    let width = WIDTH / coordinate.s;

    let m = mat2x2f(
        (coordinate.height * coordinate.s) / (2.0 * coordinate.width), 0.0,
        0.0, coordinate.s / 2.0f
    );

    let max_x = (2.0 * coordinate.width) / (coordinate.height * coordinate.s) - coordinate.center[0];
    let min_x = ( - 2.0 * coordinate.width) / (coordinate.height * coordinate.s) - coordinate.center[0];
    let max_y = 2.0 / coordinate.s - coordinate.center[1];
    let min_y = -2.0 / coordinate.s - coordinate.center[1];

    var center_x = vec2<f32>(coordinate.center[0],0.0);

    var center_y = vec2<f32>(0.0,coordinate.center[1]);

    var positions = array<vec2<f32>, 24>(
        vec2<f32>(min_x, coordinate.center[1] - width),
        vec2<f32>(max_x, coordinate.center[1] - width),
        vec2<f32>(max_x, coordinate.center[1] + width),
        vec2<f32>(max_x, coordinate.center[1] + width),
        vec2<f32>(min_x, coordinate.center[1] + width),
        vec2<f32>(min_x, coordinate.center[1] - width),

    // ---------------------------------------------
        vec2<f32>(coordinate.center[0] + width, max_y),
        vec2<f32>(coordinate.center[0] - width, max_y),
        vec2<f32>(coordinate.center[0] - width, min_y),
        vec2<f32>(coordinate.center[0] - width, min_y),
        vec2<f32>(coordinate.center[0] + width, min_y),
        vec2<f32>(coordinate.center[0] + width, max_y),
    //
        vec2<f32>(coordinate.center[0] + width - 2.0 , max_y),
        vec2<f32>(coordinate.center[0] - width - 2.0, max_y),
        vec2<f32>(coordinate.center[0] - width - 2.0, min_y),
        vec2<f32>(coordinate.center[0] - width - 2.0, min_y),
        vec2<f32>(coordinate.center[0] + width - 2.0, min_y),
        vec2<f32>(coordinate.center[0] + width - 2.0, max_y),
    //
        vec2<f32>(min_x, coordinate.center[1] - width + 2.0),
        vec2<f32>(max_x, coordinate.center[1] - width + 2.0),
        vec2<f32>(max_x, coordinate.center[1] + width + 2.0),
        vec2<f32>(max_x, coordinate.center[1] + width + 2.0),
        vec2<f32>(min_x, coordinate.center[1] + width + 2.0),
        vec2<f32>(min_x, coordinate.center[1] - width + 2.0),
        );

    var colors = vec3<f32>(0.0, 0.0, 0.0);

    var position = m * (positions[in_vertex_index] + coordinate.center);

    out.position = vec4<f32>(position , 0.0, 1.0);
    out.color = colors;

    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
