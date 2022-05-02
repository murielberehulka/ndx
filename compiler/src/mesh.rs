use wgpu::util::DeviceExt;

pub struct Mesh {
    pub vertices: crate::Vertices
}

pub fn load(device: &wgpu::Device, b: &[u8], _i: &mut usize) -> Mesh {
    let mut i = *_i;
    let use_normals = b[i+4] == b'1';
    let use_uvs = b[i+5] == b'1';
    let use_skeleton = b[i+6] == b'1';
    let vertices_length = u16::from_be_bytes([b[i+7], b[i+8]]) as usize;
    println!("Loading mesh: \n\tuse normals: {}\n\tuse uvs: {}\n\tuse skeleton: {}\n\tvertices: {}",
        use_normals, use_uvs, use_skeleton, vertices_length);
    i += 9;
    let mut vid = 0;
    let vertices = match (use_normals, use_uvs, use_skeleton) {
        (true, false, false) => {
            let mut vertices = vec![];
            while vid < vertices_length {
                vertices.push(crate::vertices::V_NORMAL {
                    position: [
                        f32::from_be_bytes([b[i], b[i+1], b[i+2], b[i+3]]),
                        f32::from_be_bytes([b[i+4], b[i+5], b[i+6], b[i+7]]),
                        f32::from_be_bytes([b[i+8], b[i+9], b[i+10], b[i+11]])
                    ],
                    normal: [
                        f32::from_be_bytes([b[i+12], b[i+13], b[i+14], b[i+15]]),
                        f32::from_be_bytes([b[i+16], b[i+17], b[i+18], b[i+19]]),
                        f32::from_be_bytes([b[i+20], b[i+21], b[i+22], b[i+23]])
                    ]
                });
                vid += 1;
                i += 24;
            }
            crate::Vertices {
                vtype: crate::VertexType::Normal,
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX
                }),
                len: vertices.len()
            }
        },
        (true, true, false) => {
            let mut vertices = vec![];
            while vid < vertices_length {
                vertices.push(crate::vertices::V_NORMAL_UV {
                    position: [
                        f32::from_be_bytes([b[i], b[i+1], b[i+2], b[i+3]]),
                        f32::from_be_bytes([b[i+4], b[i+5], b[i+6], b[i+7]]),
                        f32::from_be_bytes([b[i+8], b[i+9], b[i+10], b[i+11]])
                    ],
                    normal: [
                        f32::from_be_bytes([b[i+12], b[i+13], b[i+14], b[i+15]]),
                        f32::from_be_bytes([b[i+16], b[i+17], b[i+18], b[i+19]]),
                        f32::from_be_bytes([b[i+20], b[i+21], b[i+22], b[i+23]])
                    ],
                    uv: [
                        f32::from_be_bytes([b[i+24], b[i+25], b[i+26], b[i+27]]),
                        f32::from_be_bytes([b[i+28], b[i+29], b[i+30], b[i+31]])
                    ]
                });
                vid += 1;
                i += 32;
            }
            crate::Vertices {
                vtype: crate::VertexType::NormalUV,
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX
                }),
                len: vertices.len()
            }
        },
        _ => panic!("Wrong asset VertexType")
    };
    *_i = i;
    Mesh {
        vertices
    }
}