struct VertexInput {
    [[location(0)]] position: vec4<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] background: vec4<f32>;
    [[location(3)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] background: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vertex.position;
    out.color = vertex.color;
    out.background = vertex.background;
    out.uv = vertex.uv;
    return out;
}

[[group(0), binding(0)]]
var font: texture_2d<f32>;
[[group(0), binding(1)]]
var font_samp: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let texture = textureSample(font, font_samp, in.uv);
    if(texture.r < 1.0){
        return in.background;
    }else {
        return in.color;
    }
}