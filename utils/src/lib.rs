#[macro_export]
macro_rules! env_u32 {
    ($s: literal) => {
        utils::str_to_u32(env!($s))
    }
}

pub const fn str_to_u32(s: &'static str) -> u32 {
    let v = s.as_bytes();
    let mut res = (v[v.len() - 1] - 48) as u32;
    let mut i = 0;
    while i < v.len() {
        res += (v[i as usize] - 48) as u32 * (((v.len()-i-1) * 10) as u32);
        i += 1;
    }
    res
}