// vertex.wgsl
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct Coordinate {
    transform: f32,
    s: f32,
    center: vec2<f32>
};

@group(0) @binding(0) var<uniform> coordinate: Coordinate;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let m = mat2x2f(
        coordinate.s , 0.0,
        0.0, coordinate.s * coordinate.transform
    );

    var out: VertexOutput;

    var center_x = vec2<f32>(coordinate.center[0],0.0);

    var center_y = vec2<f32>(0.0,coordinate.center[1]);

    var positions = array<vec2<f32>, 12>(
        vec2<f32>( -0.001,  1.0) + center_x,
        vec2<f32>(-0.001, -1.0) + center_x,
        vec2<f32>( 0.001, -1.0) + center_x,
        vec2<f32>( 0.001,  -1.0) + center_x,
        vec2<f32>(0.001, 1.0) + center_x,
        vec2<f32>( -0.001, 1.0) + center_x,
    // ---------------------------------------------
        vec2<f32>( -1.0,  0.001) + center_y,
        vec2<f32>( -1.0, -0.001) + center_y,
        vec2<f32>(  1.0, -0.001) + center_y,
        vec2<f32>(  1.0, -0.001) + center_y,
        vec2<f32>(  1.0,  0.001) + center_y,
        vec2<f32>( -1.0,  0.001) + center_y,
        );

    var colors = vec3<f32>(0.0, 0.0, 0.0);

    out.position = vec4<f32>(positions[in_vertex_index] , 0.0, 1.0);
    out.color = colors;

    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
