struct Camera {
    [[location(0)]] perspective: mat4x4<f32>;
    [[location(1)]] position: vec4<f32>;
};
[[group(0), binding(0)]]
var<uniform> camera: Camera;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};
struct InstanceInput {
    [[location(3)]] position: vec3<f32>;
    [[location(4)]] rotation: vec4<f32>;
    [[location(5)]] size: f32;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = vertex.normal;
    out.uv = vertex.uv;
    let pos_rot_z = vertex.position.z * instance.rotation[0] - vertex.position.y * instance.rotation[1];
    let position = vec3<f32>(
        (vertex.position[0] * instance.rotation[2] - pos_rot_z * instance.rotation[3] + instance.position.x) * instance.size,
        (vertex.position.y * instance.rotation[0] + vertex.position.z * instance.rotation[1] + instance.position.y) * instance.size,
        (vertex.position[0] * instance.rotation[3] + pos_rot_z * instance.rotation[2] + instance.position.z) * instance.size
    );
    out.position = camera.perspective * vec4<f32>(position, 1.0);
    return out;
}

[[group(1), binding(0)]]
var diffuse: texture_2d<f32>;
[[group(1), binding(1)]]
var diffuse_samp: sampler;

let light_direction = vec3<f32> (0.0, 0.5, 0.5);

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let texture = textureSample(diffuse, diffuse_samp, in.uv);
    let normal_shadow = dot(in.normal, light_direction);
    return vec4<f32>(
        texture.r * normal_shadow,
        texture.g * normal_shadow,
        texture.b * normal_shadow,
        1.0
    );
}