use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub color: [f32;4],
    pub background: [f32;4],
    pub uv: [f32; 2]
}
pub const LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x2]
};

pub struct Text {
    pub style: super::Style,
    pub text: String,
    pub font_color: super::Color,
    pub font_size: u32
}
impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::from("Hello World !"),
            font_color: super::Color::Raw(1.0, 1.0, 1.0, 1.0),
            font_size: 21,
            style: super::Style::default()
        }
    }
}
impl super::ElementTrait for Text {
    fn element(self) -> super::Element {
        super::Element::Text(self)
    }
}
impl Text {
    fn get_vertices(
        &self,
        fonts: &super::Fonts,
        style: &super::Style,
        elements: &Vec<super::Element>
    ) -> Vec<Vertex> {
        let [px, py] = style.position_center(elements);
        let w = self.font_size as f32 / elements[0].style().width as f32;
        println!("{}", elements[0].style().width);
        let h = self.font_size as f32 / elements[0].style().height as f32;
        let text_color = self.font_color.as_raw();
        let background = match &style.background {
            crate::Background::Solid(v) => v.as_raw()
        };
        let z = style.z_index;
        let mut x = px as f32;
        let mut vertices = vec![];
        for c in self.text.chars() {
            let [cxi, cxf] = fonts.monospace.get(c);
            vertices.extend(vec![
                Vertex {
                    position: [ x, py - h, z, 1.0 ],
                    color: text_color,
                    background,
                    uv: [*cxi, 1.0]
                },
                Vertex {
                    position: [ x + w, py - h, z, 1.0 ],
                    color: text_color,
                    background,
                    uv: [*cxf, 1.0]
                },
                Vertex {
                    position: [ x, py, z, 1.0 ],
                    color: text_color,
                    background,
                    uv: [*cxi, 0.0]
                },
                Vertex {
                    position: [ x + w, py, z, 1.0 ],
                    color: text_color,
                    background,
                    uv: [*cxf, 0.0]
                }
            ]);
            x += w;
        }
        vertices
    }
}

pub struct Shader {
    pub render_pipeline: wgpu::RenderPipeline,
    pub needs_update: bool,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub buffer: Option<(wgpu::Buffer, wgpu::Buffer)>, //vertices, indices
}

impl Shader {
    pub fn new(
        device: &wgpu::Device,
        texture_diffuse_bgl: &wgpu::BindGroupLayout,
        texture_format: wgpu::TextureFormat
    ) -> Shader {
        let shader = device.create_shader_module(&wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    texture_diffuse_bgl
                ],
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
                depth_compare: wgpu::CompareFunction::LessEqual,
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
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, fonts: &'a super::Fonts) {
        if let Some((vertices, indices)) = &self.buffer {
            render_pass.set_bind_group(0, &fonts.monospace.texture.bind_group, &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, vertices.slice(..));
            render_pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
        }
    }
}