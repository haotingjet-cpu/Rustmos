// vertex.wgsl
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
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

    out.position = vec4<f32>(positions[in_vertex_index], 0.0, 1.0);
    out.color = colors[in_vertex_index];

    return out;
}

// 修正這裡：讓 fragment 接收 VertexOutput 作為輸入參數
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 讀取內插後的顏色 (in.color)，並加上透明度 1.0
    return vec4<f32>(in.color, 1.0);
}
