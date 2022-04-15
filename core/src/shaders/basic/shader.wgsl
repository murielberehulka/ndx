struct Camera {
    [[location(0)]] perspective: mat4x4<f32>;
    [[location(1)]] position: vec4<f32>;
};
[[group(0), binding(0)]]
var<uniform> camera: Camera;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
};
struct InstanceInput {
    [[location(2)]] position: vec3<f32>;
    [[location(3)]] rotation: vec4<f32>;
    [[location(4)]] size: f32;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = vertex.normal;
    let pos_rot_z = vertex.position.z * instance.rotation[0] - vertex.position.y * instance.rotation[1];
    let position = vec3<f32>(
        (vertex.position[0] * instance.rotation[2] - pos_rot_z * instance.rotation[3] + instance.position.x) * instance.size,
        (vertex.position.y * instance.rotation[0] + vertex.position.z * instance.rotation[1] + instance.position.y) * instance.size,
        (vertex.position[0] * instance.rotation[3] + pos_rot_z * instance.rotation[2] + instance.position.z) * instance.size
    );
    out.position = camera.perspective * vec4<f32>(position, 1.0);
    return out;
}

struct Material {
    [[location(0)]] color: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> material: Material;

let light_direction = vec3<f32> (0.0, 0.5, 0.25);

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let normal_shadow = dot(in.normal, light_direction);
    return vec4<f32>(
        material.color.r * normal_shadow,
        material.color.g * normal_shadow,
        material.color.b * normal_shadow,
        1.0
    );
}