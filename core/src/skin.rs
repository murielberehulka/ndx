use wgpu::util::DeviceExt;

pub type Joints = [crate::Joint;crate::MAX_IBMS];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkinRaw {
    pub mats: [[f32; 16];crate::MAX_IBMS]
}

#[derive(Copy, Clone)]
pub struct Joint {
    pub id: usize,
    pub ibm: [f32;16],
    pub translation: [f32;3],
    pub rotation: Option<[f32;3]>,
    pub parent: usize
}
impl Joint {
    pub fn get_mat(&self, joints: &Joints) -> math::Mat4 {
        let transform = match self.rotation {
            Some(rotation) => math::mat4::from_tr(self.translation, rotation),
            None => math::mat4::from_translation(self.translation)
        };
        if self.id != 0 {
            math::mat4::mul_mat4(joints[self.parent].get_mat(joints), transform)
        }else {
            transform
        }
    }
}

pub struct MeshSkin {
    pub joints: Joints
}

pub struct InstanceSkin {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub raw: SkinRaw
}
impl InstanceSkin {
    pub fn new(
        device: &wgpu::Device,
        bgls: &crate::Bgls,
        mesh_skin_joints: &Joints
    ) -> Self {
        let raw = Self::get_raw(mesh_skin_joints);
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[raw]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bgls.skin,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });
        Self {
            bind_group,
            buffer,
            raw
        }
    }
    pub fn get_raw(joints: &Joints) -> SkinRaw {
        let mut mats = [[0.0;16];crate::MAX_IBMS];
        for (i, joint) in joints.iter().enumerate() {
            mats[i] = math::mat4::mul_mat4(joint.get_mat(joints), joint.ibm);
        }
        SkinRaw {
            mats
        }
    }
}