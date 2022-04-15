use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub color: [f32; 4]
}
pub const LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4]
};

pub struct Rect {
    pub style: super::Style
}
impl super::ElementTrait for Rect {
    fn element(self) -> super::Element {
        super::Element::Rect(self)
    }
}
impl Rect {
    pub fn get_vertices(
        &self,
        elements: &Vec<super::Element>
    ) -> Vec<Vertex> {
        let [x, y] = self.style.position_center(elements);
        let w = self.style.width as f32 / elements[0].style().width as f32;
        let h = self.style.height as f32 / elements[0].style().width as f32;
        let color = match &self.style.background {
            crate::Background::Solid(v) => v.as_raw()
        };
        let z = self.style.z_index;
        vec![
            Vertex {
                position: [ x, y - h, z, 1.0 ],
                color
            },
            Vertex {
                position: [ x + w, y - h, z, 1.0 ],
                color
            },
            Vertex {
                position: [ x, y, z, 1.0 ],
                color
            },
            Vertex {
                position: [ x + w, y, z, 1.0 ],
                color
            }
        ]
    }
}

pub struct Shader {
    pub render_pipeline: wgpu::RenderPipeline,
    pub needs_update: bool,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub buffer: Option<(wgpu::Buffer, wgpu::Buffer)> //vertices, indices
}

impl Shader {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat
    ) -> Shader {
        let shader = device.create_shader_module(&wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[]
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[LAYOUT]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL
                }]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::depth::FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });
        Shader {
            render_pipeline,
            needs_update: false,
            vertices: vec![],
            indices: vec![],
            buffer: None
        }
    }
    pub fn reset(&mut self) {
        self.vertices = vec![];
        self.indices = vec![];
    }
    pub fn add_vertices(&mut self, vertices: [Vertex;4]) {
        let i = self.vertices.len();
        self.vertices.extend(vertices);
        self.indices.extend([i as u16, (i+1)as u16, (i+3)as u16, i as u16, (i+3)as u16, (i+2)as u16]);
    }
    pub fn update(&mut self, device: &wgpu::Device) {
        self.buffer = Some((
            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::VERTEX
                }
            ),
            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::INDEX
                }
            )
        ))
    }
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some((vertices, indices)) = &self.buffer {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, vertices.slice(..));
            render_pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
        }
    }
}