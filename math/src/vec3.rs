pub type Vec3 = [f32;3];

#[inline]
pub fn rotate_y(p: [f32;3], a: f32) -> [f32;3] {
    let cos = a.cos();
    let sin = a.sin();
    [
        p[0] * cos - p[2] * sin,
        p[1],
        p[0] * sin + p[2] * cos
    ]
}
#[inline]
pub fn rotate_x(p: [f32;3], a: f32) -> [f32;3] {
    let cos = a.cos();
    let sin = a.sin();
    [
        p[0],
        p[1] * cos + p[2] * sin,
        p[2] * cos - p[1] * sin
    ]
}
#[inline]
pub fn rotate_xy(p: [f32;3], ax: f32, ay: f32) -> [f32;3] {
    let cos_x = ax.cos();
    let sin_x = ax.sin();
    let cos_y = ay.cos();
    let sin_y = ay.sin();
    let z = p[2] * cos_x - p[1] * sin_x;
    [
        p[0] * cos_y - z * sin_y,
        p[1] * cos_x + z * sin_x,
        p[0] * sin_y + z * cos_y
    ]
}
#[inline]
pub fn normalize(v: Vec3) -> Vec3 {
    let l = ((v[0]*v[0])+(v[1]*v[1])+(v[2]*v[2])).sqrt();
    [v[0] / l, v[1] / l, v[2] / l]
}
#[inline]
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        (a[1] * b[2]) - (a[2] * b[1]),
        (a[2] * b[0]) - (a[0] * b[2]),
        (a[0] * b[1]) - (a[1] * b[0])
    ]
}
#[inline]
pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}