// vertex.wgsl
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct Coordinate {
    transform: vec2<f32>,
    s: f32,
    _pad: u32
};

@group(0) @binding(0) var<uniform> coordinate: Coordinate;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let m = mat2x2f(
        coordinate.s, 0.0,
        0.0, coordinate.s * coordinate.transform[0] / coordinate.transform[1]
    );

    var out: VertexOutput;

    var positions = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5)
    );

    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0), // 紅
        vec3<f32>(0.0, 1.0, 0.0), // 綠
        vec3<f32>(0.0, 0.0, 1.0)  // 藍
    );

    out.position = vec4<f32>(m * positions[in_vertex_index], 0.0, 1.0);
    out.color = colors[in_vertex_index];

    return out;
}

// 修正這裡：讓 fragment 接收 VertexOutput 作為輸入參數
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 讀取內插後的顏色 (in.color)，並加上透明度 1.0
    return vec4<f32>(in.color, 1.0);
}
