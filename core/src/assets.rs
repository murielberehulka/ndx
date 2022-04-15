use wgpu::util::DeviceExt;

pub struct Assets {
    pub meshes: Vec<crate::Mesh>
}

impl Assets {
    pub fn new() -> Self {
        Self {
            meshes: vec![]
        }
    }
    pub fn load(&mut self, device: &wgpu::Device) {
        let b = std::fs::read("./.assets/compiled.info").expect("Can not load the assets");
        let mut i = 0;
        loop {
            match (b[i], b[i+1], b[i+2], b[i+3]) {
                // MESH
                (109, 101, 115, 104) => {
                    let positions_length = u16::from_be_bytes([b[i+4], b[i+5]]) as usize;
                    i += 6;
                    let j = i + (positions_length * 12);
                    let mut positions: Vec<[f32;3]> = Vec::with_capacity(positions_length);
                    while i < j {
                        positions.push([
                            f32::from_be_bytes([b[i],b[i+1],b[i+2],b[i+3]]),
                            f32::from_be_bytes([b[i+4],b[i+5],b[i+6],b[i+7]]),
                            f32::from_be_bytes([b[i+8],b[i+9],b[i+10],b[i+11]])
                        ]);
                        i += 12;
                    }
                    let indices_length = u16::from_be_bytes([b[i], b[i+1]]) as usize;
                    i += 2;
                    let j = i + (indices_length * 2);
                    let mut indices: Vec<u16> = Vec::with_capacity(positions_length);
                    while i < j {
                        indices.push(u16::from_be_bytes([b[i], b[i+1]]));
                        i += 2;
                    }
                    let mut normals: Option<Vec<[f32;3]>> = None;
                    if b[i] == 110 && b[i+1] == 114 && b[i+2] == 109 && b[i+3] == 108 {
                        i += 4;
                        let mut vec = Vec::with_capacity(positions_length);
                        let j = i + (positions_length * 12);
                        while i < j {
                            vec.push([
                                f32::from_be_bytes([b[i],b[i+1],b[i+2],b[i+3]]),
                                f32::from_be_bytes([b[i+4],b[i+5],b[i+6],b[i+7]]),
                                f32::from_be_bytes([b[i+8],b[i+9],b[i+10],b[i+11]])
                            ]);
                            i += 12
                        }
                        normals = Some(vec);
                    }
                    let mut uvs: Option<Vec<[f32;2]>> = None;
                    if b[i] == 117 && b[i+1] == 118 && b[i+2] == 115 && b[i+3] == 95 {
                        i += 4;
                        let mut vec = Vec::with_capacity(positions_length);
                        let j = i + (positions_length * 8);
                        while i < j {
                            vec.push([
                                f32::from_be_bytes([b[i],b[i+1],b[i+2],b[i+3]]),
                                f32::from_be_bytes([b[i+4],b[i+5],b[i+6],b[i+7]])
                            ]);
                            i += 8
                        }
                        uvs = Some(vec);
                    }
                    let mut joint_indices: Option<Vec<[u32;4]>> = None;
                    if b[i] == 106 && b[i+1] == 110 && b[i+2] == 116 && b[i+3] == 115 {
                        i += 4;
                        let mut vec = Vec::with_capacity(positions_length);
                        let j = i + (positions_length * 8);
                        while i < j {
                            vec.push([
                                u16::from_be_bytes([b[i],b[i+1]]) as u32,
                                u16::from_be_bytes([b[i+2],b[i+3]]) as u32,
                                u16::from_be_bytes([b[i+4],b[i+5]]) as u32,
                                u16::from_be_bytes([b[i+6],b[i+7]]) as u32
                            ]);
                            i += 8
                        }
                        joint_indices = Some(vec);
                    }
                    let mut weights: Option<Vec<[f32;4]>> = None;
                    if b[i] == 119 && b[i+1] == 103 && b[i+2] == 104 && b[i+3] == 116 {
                        i += 4;
                        let mut vec = Vec::with_capacity(positions_length);
                        let j = i + (positions_length * 16);
                        while i < j {
                            vec.push([
                                f32::from_be_bytes([b[i],b[i+1],b[i+2],b[i+3]]),
                                f32::from_be_bytes([b[i+4],b[i+5],b[i+6],b[i+7]]),
                                f32::from_be_bytes([b[i+8],b[i+9],b[i+10],b[i+11]]),
                                f32::from_be_bytes([b[i+12],b[i+13],b[i+14],b[i+15]])
                            ]);
                            i += 16
                        }
                        weights = Some(vec);
                    }
                    let mut joints: Option<Vec<([f32;16], [f32;3], u16)>> = None; // inverse bind matrix, position, parent id
                    if b[i] == 115 && b[i+1] == 107 && b[i+2] == 105 && b[i+3] == 110 {
                        let joints_length = u16::from_be_bytes([b[i+4], b[i+5]]) as usize;
                        i += 6;
                        let mut vec = Vec::with_capacity(joints_length);
                        let j = i + (joints_length * 78);
                        while i < j {
                            vec.push((
                                [
                                    f32::from_be_bytes([b[i],b[i+1],b[i+2],b[i+3]]),
                                    f32::from_be_bytes([b[i+4],b[i+5],b[i+6],b[i+7]]),
                                    f32::from_be_bytes([b[i+8],b[i+9],b[i+10],b[i+11]]),
                                    f32::from_be_bytes([b[i+12],b[i+13],b[i+14],b[i+15]]),
                                    f32::from_be_bytes([b[i+16],b[i+17],b[i+18],b[i+19]]),
                                    f32::from_be_bytes([b[i+20],b[i+21],b[i+22],b[i+23]]),
                                    f32::from_be_bytes([b[i+24],b[i+25],b[i+26],b[i+27]]),
                                    f32::from_be_bytes([b[i+28],b[i+29],b[i+30],b[i+31]]),
                                    f32::from_be_bytes([b[i+32],b[i+33],b[i+34],b[i+35]]),
                                    f32::from_be_bytes([b[i+36],b[i+37],b[i+38],b[i+39]]),
                                    f32::from_be_bytes([b[i+40],b[i+41],b[i+42],b[i+43]]),
                                    f32::from_be_bytes([b[i+44],b[i+45],b[i+46],b[i+47]]),
                                    f32::from_be_bytes([b[i+48],b[i+49],b[i+50],b[i+51]]),
                                    f32::from_be_bytes([b[i+52],b[i+53],b[i+54],b[i+55]]),
                                    f32::from_be_bytes([b[i+56],b[i+57],b[i+58],b[i+59]]),
                                    f32::from_be_bytes([b[i+60],b[i+61],b[i+62],b[i+63]])
                                ],
                                [
                                    f32::from_be_bytes([b[i+64],b[i+65],b[i+66],b[i+67]]),
                                    f32::from_be_bytes([b[i+68],b[i+69],b[i+70],b[i+71]]),
                                    f32::from_be_bytes([b[i+72],b[i+73],b[i+74],b[i+75]])
                                ],
                                u16::from_be_bytes([b[i+76], b[i+77]])
                            ));
                            i += 78
                        }
                        joints = Some(vec);
                    }
                    let index_buffer = device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::cast_slice(&indices),
                            usage: wgpu::BufferUsages::INDEX
                        }
                    );
                    match (normals, uvs, joint_indices, weights, joints) {
                        (Some(normals), Some(uvs), None, None, None) => {
                            let mut vertices: Vec<crate::vertices::NUV> = Vec::with_capacity(positions_length);
                            for i in 0..positions.len() {
                                vertices.push(crate::vertices::NUV {
                                    position: positions[i],
                                    normal: normals[i],
                                    uv: uvs[i]
                                })
                            }
                            self.meshes.push(crate::Mesh {
                                vertex_type: crate::VertexType::NormalTexture,
                                vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: None,
                                    contents: bytemuck::cast_slice(&vertices),
                                    usage: wgpu::BufferUsages::VERTEX
                                }),
                                index_buffer,
                                indices_len: indices.len() as u32,
                                models: vec![],
                                skin: None
                            })
                        },
                        (Some(normals), Some(uvs), Some(joint_indices), Some(weights), Some(joints)) => {
                            let mut vertices: Vec<crate::vertices::NUVS> = Vec::with_capacity(positions_length);
                            for i in 0..positions.len() {
                                vertices.push(crate::vertices::NUVS {
                                    position: positions[i],
                                    normal: normals[i],
                                    uv: uvs[i],
                                    joints: joint_indices[i],
                                    weights: weights[i]
                                })
                            }
                            let mut skin_joints: [crate::Joint; crate::MAX_IBMS] = [crate::Joint {
                                id: 0,
                                ibm: [0.0;16],
                                translation: [0.0;3],
                                rotation: None,
                                parent: 0
                            };crate::MAX_IBMS];
                            for (i, (ibm, translation, parent)) in joints.iter().enumerate() {
                                skin_joints[i] = crate::Joint {
                                    id: i,
                                    ibm: *ibm,
                                    translation: *translation,
                                    rotation: None,
                                    parent: *parent as usize
                                };
                            }
                            self.meshes.push(crate::Mesh {
                                vertex_type: crate::VertexType::NormalTextureSkin,
                                vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: None,
                                    contents: bytemuck::cast_slice(&vertices),
                                    usage: wgpu::BufferUsages::VERTEX
                                }),
                                index_buffer,
                                indices_len: indices.len() as u32,
                                models: vec![],
                                skin: Some(crate::MeshSkin {
                                    joints: skin_joints
                                })
                            })
                        },
                        _ => panic!("Unsupported mesh format")
                    }
                },
                // END
                (101, 110, 100, 95) => break,
                _ => panic!("Asset type {} not supported", String::from_utf16_lossy(&[b[i]as u16, b[i+1]as u16, b[i+2]as u16, b[i+3]as u16]))
            }
        }
    }
}