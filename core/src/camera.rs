use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraRaw {
    pub perspective: [f32; 16],
    pub position: [f32; 4]
}

pub struct Camera {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    needs_update: bool,
    raw: CameraRaw,
    position: [f32;3],
    target: [f32;3],
    fov: f32,
    near: f32,
    far: f32,
    aspect: f32
}

impl Camera {
    pub fn new(
        s: &crate::CameraSettings,
        device: &wgpu::Device,
        window_size: &winit::dpi::PhysicalSize<u32>,
        bgls: &crate::Bgls
    ) -> Self {
        let aspect = (window_size.width / window_size.height) as f32;
        let raw = Self::get_raw(s.position, s.target, s.fov, s.near, s.far, aspect);
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[raw]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bgls.camera,
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
            raw,
            needs_update: false,
            position: s.position,
            target: s.target,
            fov: s.fov,
            near: s.near,
            far: s.far,
            aspect
        }
    }
    pub fn get_raw(
        position: [f32;3],
        target: [f32;3],
        fov: f32,
        near: f32,
        far: f32,
        aspect: f32
    ) -> CameraRaw {
        let view = math::mat4::look_at(position, target);
        let proj = math::mat4::perspective(math::deg(fov), aspect, near, far);
        let perspective = math::mat4::mul_mat4(proj, view);
        CameraRaw {
            perspective,
            position: [position[0], position[1], position[2], 1.0]
        }
    }
    pub fn update(&mut self, queue: &wgpu::Queue) {
        if self.needs_update {
            self.raw = Self::get_raw(
                self.position,
                self.target,
                self.fov,
                self.near,
                self.far,
                self.aspect
            );
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
            self.needs_update = false;
        }
    }
    pub fn add_fov(&mut self, v: f32) {
        self.fov += v;
        if self.fov < 1.0 { self.fov = 1.0 }
        self.needs_update = true;
    }
    pub fn set_position(&mut self, v: [f32;3]) {
        self.position = v;
        self.needs_update = true;
    }
    pub fn set_aspect(&mut self, v: f32) {
        self.aspect = if v != 0.0 {v} else {0.01};
        self.needs_update = true;
    }
    pub fn translate(&mut self, v: [f32;3]) {
        self.position[0] += v[0];
        self.position[1] += v[1];
        self.position[2] += v[2];
        self.needs_update = true;
    }
    pub fn get_position(&self) -> &[f32;3] {
        &self.position
    }
    pub fn set_target(&mut self, v: [f32;3]) {
        self.target = v;
        self.needs_update = true;
    }
}