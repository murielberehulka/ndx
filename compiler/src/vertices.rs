pub struct Vertices {
    pub vtype: VertexType,
    pub buffer: wgpu::Buffer,
    pub len: usize
}

pub enum VertexType {
    Basic,
    Normal,
    NormalUV,
    NormalUVSkin
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct V_NORMAL {
    pub position: [f32; 3],
    pub normal: [f32; 3]
}
pub const V_LAYOUT_NORMAL: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<V_NORMAL>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3]
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct V_NORMAL_UV {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2]
}
pub const V_LAYOUT_NORMAL_UV: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<V_NORMAL_UV>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2]
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct V_NORMAL_UV_SKIN {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub joints: [u32;4],
    pub weights: [f32;4]
}
pub const V_LAYOUT_NORMAL_UV_SKIN: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<V_NORMAL_UV_SKIN>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Uint32x4, 4 => Float32x4]
};