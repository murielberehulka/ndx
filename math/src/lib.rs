#[rustfmt::skip]
pub mod mat4;
pub use mat4::Mat4;
#[rustfmt::skip]
pub mod vec3;
pub use vec3::Vec3;

pub fn deg(v: f32) -> f32 {
    v / 180.0 * 3.14159265
}