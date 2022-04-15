use wgpu::util::DeviceExt;

pub enum MaterialRaw {
    Basic (MaterialRawBasic)
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialRawBasic {
    color: [f32; 4]
}

pub struct Material {
    pub shader: crate::Shader,
    pub buffer: Option<wgpu::Buffer>,
    pub bind_group: Option<wgpu::BindGroup>
}
impl Material {
    pub fn new(device: &wgpu::Device, bgls: &crate::Bgls, shader: crate::Shader) -> Self {
        let raw = get_material_raw_from_shader(&shader);
        let (buffer,bind_group) = match raw {
            Some(raw) => match raw  {
                MaterialRaw::Basic (v) => {
                    let buffer = get_buffer(device, v);
                    let bind_group = get_bind_group(device, &bgls.material_basic, &buffer);
                    (Some(buffer), Some(bind_group))
                }
            }
            None => (None, None)
        };
        Self {
            shader,
            buffer,
            bind_group
        }
    }
}

fn get_buffer(device: &wgpu::Device, v: impl bytemuck::Pod) -> wgpu::Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[v]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    )
}
fn get_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, buffer: &wgpu::Buffer) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding()
            }
        ]
    })
}
pub fn get_material_raw_from_shader(shader: &crate::Shader) -> Option<MaterialRaw> {
    match shader {
        crate::Shader::Basic { color } =>
            Some(MaterialRaw::Basic(MaterialRawBasic {
                color: [color[0], color[1], color[2], 1.0]
            })),
        crate::Shader::BasicDiffuseTexture {..} =>
            None,
        crate::Shader::AnimatedDiffuseTexture {..} =>
            None
    }
}