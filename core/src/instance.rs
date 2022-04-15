use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub position: [f32;3],
    pub rotation: [f32;4],
    pub size: f32
}

pub struct Instances {
    pub raws: Vec<InstanceRaw>,
    pub buffer: Option<wgpu::Buffer>,
    pub needs_update: bool,
    pub skins: Option<Vec<crate::InstanceSkin>>
}

impl Instances {
    pub fn new() -> Self {
        Self {
            raws: vec![],
            buffer: None,
            needs_update: false,
            skins: None
        }
    }
    pub fn add(&mut self, v: InstanceRaw, skin: Option<crate::InstanceSkin>) {
        if let Some(skin) = skin {
            match &mut self.skins {
                Some(skins) => skins.push(skin),
                None => self.skins = Some(vec![skin])
            }
        }
        self.raws.push(v);
        self.needs_update = true;
    }
    pub fn remove(&mut self, id: usize) {
        self.raws.remove(id);
        self.needs_update = true;
    }
    pub fn update(&mut self, device: &wgpu::Device) {
        if self.needs_update {
            if self.raws.len() > 0 {
                self.buffer = Some(device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(&self.raws),
                        usage: wgpu::BufferUsages::VERTEX
                    }
                ));
            }else {
                self.buffer = None;
            }
            self.needs_update = false;
        }
    }
    pub fn get_instance_skin(&mut self, id: usize) -> &mut crate::InstanceSkin {
        match &mut self.skins {
            Some(skin) => &mut skin[id],
            None => panic!("Mesh has no skin")
        }
    }
}