use std::collections::HashMap;

pub struct Font {
    pub chars: HashMap<char, [f32;2]>,
    pub chars_length: usize,
    pub texture: texture::diffuse::Texture
}

impl Font {
    pub fn from(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_diffuse_bgl: &wgpu::BindGroupLayout,
        texture_bytes: &[u8],
        chars_str: &str
    ) -> Self {
        let texture = texture::diffuse::Texture::from_bytes(device, queue, texture_diffuse_bgl, texture_bytes);
        let mut chars = HashMap::new();
        let chars_length = chars_str.chars().count();
        let l = 1.0 / chars_length as f32;
        let mut i = 0.0;
        for c in chars_str.chars() {
            chars.insert(c, [l * i, l * (i + 1.0)]);
            i += 1.0;
        }
        Self {
            chars,
            chars_length,
            texture
        }
    }
    pub fn get(&self, c: char) -> &[f32;2] {
        match self.chars.get(&c) {
            Some(v) => v,
            None => panic!("Char '{}' not implemented", c)
        }
    }
}

pub struct Fonts {
    pub monospace: Font
}
impl Fonts {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, texture_diffuse_bgl: &wgpu::BindGroupLayout) -> Self {
        Self {
            monospace: Font::from(
                device, queue, texture_diffuse_bgl,
                include_bytes!("../../font_raster/monospace.png"),
                "█abcdefghijklmnopqrstuvxywzçáéíóúàèìòùãõâêîôûABCDEFGHIJKLMNOPQRSTUVXYWZÇÁÉÍÓÚÀÈÌÒÙÃÕÂÊÎÔÛ!@#$%&*()<>{}~^;:,./\\][|-+=_\"' █"
            )
        }
    }
}