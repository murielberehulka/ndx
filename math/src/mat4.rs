use super::vec3;

/// ### row major 4x4 matrix
pub type Mat4 = [f32;16];

pub const ZERO: [f32;16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

#[inline]
pub fn from_translation(v: [f32;3]) -> Mat4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        v[0],   v[1],   v[2],   1.0
    ]
}
#[inline]
pub fn from_size(v: f32) -> Mat4 {
    [
        v  , 0.0, 0.0, 0.0,
        0.0, v  , 0.0, 0.0,
        0.0, 0.0, v  , 0.0,
        0.0, 0.0, 0.0, 1.0
    ]
}
#[inline]
pub fn from_scale(x: f32, y: f32, z: f32) -> Mat4 {
    [
        x  , 0.0, 0.0, 0.0,
        0.0, y  , 0.0, 0.0,
        0.0, 0.0, z  , 0.0,
        0.0, 0.0, 0.0, 1.0
    ]
}
#[inline]
pub fn from_rotation(x: f32, y: f32, z: f32) -> Mat4 {
    let xc = x.cos();
    let xs = x.sin();
    let yc = y.cos();
    let ys = y.sin();
    let zc = z.cos();
    let zs = z.sin();
    let xsys = xs*ys;
    let xcys = -xc*ys;
    [
        yc*zc, yc*-zs, ys, 0.0,
        (xsys*zc)+(xc*zs), (xsys*-zs)+(xc*zc), -xs*yc, 0.0,
        (xcys*zc)+(xs*zs), (xcys*-zs)+(xs*zs), xc*yc, 0.0,
        0.0, 0.0, 0.0, 1.0
    ]
}
#[inline]
pub fn mul_mat4(a: Mat4, b: Mat4) -> Mat4 {
    [
        b[0] * a[0] + b[1] * a[4] + b[2] * a[8] + b[3] * a[12],
        b[0] * a[1] + b[1] * a[5] + b[2] * a[9] + b[3] * a[13],
        b[0] * a[2] + b[1] * a[6] + b[2] * a[10] + b[3] * a[14],
        b[0] * a[3] + b[1] * a[7] + b[2] * a[11] + b[3] * a[15],
        b[4] * a[0] + b[5] * a[4] + b[6] * a[8] + b[7] * a[12],
        b[4] * a[1] + b[5] * a[5] + b[6] * a[9] + b[7] * a[13],
        b[4] * a[2] + b[5] * a[6] + b[6] * a[10] + b[7] * a[14],
        b[4] * a[3] + b[5] * a[7] + b[6] * a[11] + b[7] * a[15],
        b[8] * a[0] + b[9] * a[4] + b[10] * a[8] + b[11] * a[12],
        b[8] * a[1] + b[9] * a[5] + b[10] * a[9] + b[11] * a[13],
        b[8] * a[2] + b[9] * a[6] + b[10] * a[10] + b[11] * a[14],
        b[8] * a[3] + b[9] * a[7] + b[10] * a[11] + b[11] * a[15],
        b[12] * a[0] + b[13] * a[4] + b[14] * a[8] + b[15] * a[12],
        b[12] * a[1] + b[13] * a[5] + b[14] * a[9] + b[15] * a[13],
        b[12] * a[2] + b[13] * a[6] + b[14] * a[10] + b[15] * a[14],
        b[12] * a[3] + b[13] * a[7] + b[14] * a[11] + b[15] * a[15]
    ]
}
#[inline]
pub fn translate(a: Mat4, b: [f32;3]) -> Mat4 {
    [
        a[0] + a[12] * b[0], a[4] + a[12] * b[1], a[ 8] + a[12] * b[2], a[12],
        a[1] + a[13] * b[0], a[5] + a[13] * b[1], a[ 9] + a[13] * b[2], a[13],
        a[2] + a[14] * b[0], a[6] + a[14] * b[1], a[10] + a[14] * b[2], a[14],
        a[3] + a[15] * b[0], a[7] + a[15] * b[1], a[11] + a[15] * b[2], a[15]
    ]
}
#[inline]
pub fn scale(a: Mat4, b: [f32;3]) -> Mat4 {
    [
        a[0] * b[0], a[4] * b[1], a[ 8] * b[2], a[12],
        a[1] * b[0], a[5] * b[1], a[ 9] * b[2], a[13],
        a[2] * b[0], a[6] * b[1], a[10] * b[2], a[14],
        a[3] * b[0], a[7] * b[1], a[11] * b[2], a[15]
    ]
}
#[inline]
pub fn look_at(eye: [f32;3], center: [f32;3]) -> Mat4 {
    let meye = [-eye[0], -eye[1], -eye[2]];
    let f = vec3::normalize([
        center[0] - eye[0],
        center[1] - eye[1],
        center[2] - eye[2]
    ]);
    let s = vec3::normalize(vec3::cross(f, [0.0, 1.0, 0.0]));
    let u = vec3::cross(s,f);
    [
        s[0].clone(), u[0].clone(), -f[0].clone(), 0.0,
        s[1].clone(), u[1].clone(), -f[1].clone(), 0.0,
        s[2].clone(), u[2].clone(), -f[2].clone(), 0.0,
        vec3::dot(meye,s), vec3::dot(meye,u), vec3::dot(eye,f), 1.0
    ]
}
#[inline]
pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    assert!(fov > 0.0, "Fov cannot be below zero, found: {}", fov);
    assert!(fov > 1.0, "Fov cannot be greater than a half turn, found: {}", fov);
    assert!(aspect > 0.0, "Aspect ratio cannot be below zero, found: {}", aspect);
    assert!(near > 0.0, "Near cannot be below zero, found: {}", near);
    assert!(far > 0.0, "Far cannot be below zero, found: {}", far);
    assert!(far > near, "Far cannot be closer than Near, found: far: {}, near: {}", far, near);
    let y = 1.0 / (fov / 2.0).tan();
    let x = y / aspect;
    let nearmfar = near - far;
    [
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, (far + near) / nearmfar, -1.0,
        0.0, 0.0, 2.0*far*near / nearmfar, 0.0
    ]
}
/// Mat4 from translation * scale * rotation
#[inline]
pub fn from_tsr(t: [f32;3], s: [f32;3], r: [f32;3]) -> Mat4 {
    let xc = r[0].cos();
    let xs = r[0].sin();
    let yc = r[1].cos();
    let ys = r[1].sin();
    let zc = r[2].cos();
    let zs = r[2].sin();
    let ys_zs = ys*zs;
    let ys_xs = ys*xs;
    let ys_xc = ys*xc;
    let r = [
        yc*zc, -ys_zs, ys, 0.0,
        (ys_xs*zc)+(xc*zs), (-zs*ys_xs)+(xc*zc), -xs*yc, 0.0,
        (-ys_xc*zc)+(xs*zs), (ys_xc*zs)+(xs*zc), yc*xc, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];
    mul_mat4([
        s[0], 0.0, 0.0, 0.0,
        0.0, s[1], 0.0, 0.0,
        0.0, 0.0, s[2], 0.0,
        t[0]*s[0], t[1]*s[1], t[2]*s[2], 1.0
    ], r)
}
/// Mat4 from translation * rotation
#[inline]
pub fn from_tr(t: [f32;3], r: [f32;3]) -> Mat4 {
    let xc = r[0].cos();
    let xs = r[0].sin();
    let yc = r[1].cos();
    let ys = r[1].sin();
    let zc = r[2].cos();
    let zs = r[2].sin();
    let ys_zs = ys*zs;
    let ys_xs = ys*xs;
    let ys_xc = ys*xc;
    [
                      yc*zc,              -ys_zs,     ys, 0.0,
         (ys_xs*zc)+(xc*zs), (-zs*ys_xs)+(xc*zc), -xs*yc, 0.0,
        (-ys_xc*zc)+(xs*zs),  (ys_xc*zs)+(xs*zc),  yc*xc, 0.0,
                       t[0],                t[1],   t[2], 1.0
    ]
}