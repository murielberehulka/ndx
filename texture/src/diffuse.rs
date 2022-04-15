pub struct Texture {
    pub bind_group: wgpu::BindGroup
}
impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_diffuse_bgl: &wgpu::BindGroupLayout,
        bytes: &[u8]
    ) -> Self {
        let image = image::io::Reader::new(std::io::Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap().to_rgba8();
        Self::load(device, queue, texture_diffuse_bgl, image)
    }
    pub fn from_path(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_diffuse_bgl: &wgpu::BindGroupLayout,
        path: &std::path::Path
    ) -> Self {
        let image = image::io::Reader::open(path).unwrap().decode().unwrap().to_rgba8();
        Self::load(device, queue, texture_diffuse_bgl, image)
    }
    pub fn load(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_diffuse_bgl: &wgpu::BindGroupLayout,
        image: image::RgbaImage
    ) -> Self {
        let rgba = image.as_raw();
        let dimensions = image.dimensions();
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
        });
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_size
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: texture_diffuse_bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: None,
            }
        );
        Self {
            bind_group
        }
    }
}