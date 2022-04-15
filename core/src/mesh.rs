pub struct Mesh {
    pub vertex_type: crate::VertexType,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub indices_len: u32,
    pub models: Vec<crate::Model>,
    pub skin: Option<crate::MeshSkin>
}