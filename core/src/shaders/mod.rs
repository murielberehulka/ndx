pub mod basic;
pub mod basic_diftex;
pub mod anim_diftex;

pub enum Shader {
    Basic {
        color: [f32;3]
    },
    BasicDiffuseTexture {
        diffuse_texture_id: usize
    },
    AnimatedDiffuseTexture {
        diffuse_texture_id: usize
    }
}

pub struct Shaders {
    pub basic: basic::Shader,
    pub basic_diftex: basic_diftex::Shader,
    pub anim_diftex: anim_diftex::Shader
}

impl Shaders {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        bgls: &crate::Bgls
    ) -> Self {
        Self {
            basic: basic::Shader::new(&device, texture_format, bgls),
            basic_diftex: basic_diftex::Shader::new(&device, texture_format, bgls),
            anim_diftex: anim_diftex::Shader::new(&device, texture_format, bgls)
        }
    }
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &'a wgpu::BindGroup,
        meshes: &'a[crate::Mesh],
        diffuse_textures: &'a [texture::diffuse::Texture]
    ) {
        render_pass.set_pipeline(&self.basic.render_pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        for mesh in meshes {
            for model in &mesh.models {
                if let crate::Shader::Basic {..} = model.material.shader {
                    if let Some(instance_buffer) = &model.instances.buffer {
                        render_pass.set_bind_group(1, model.material.bind_group.as_ref().unwrap(), &[]);
                        render_pass.set_vertex_buffer(0, mesh.vertices.buffer.slice(..));
                        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                        render_pass.draw(0..mesh.vertices.len as u32, 0..model.instances.raws.len() as u32);
                    }
                }
            }
        }
        render_pass.set_pipeline(&self.basic_diftex.render_pipeline);
        for mesh in meshes {
            for model in &mesh.models {
                if let crate::Shader::BasicDiffuseTexture { diffuse_texture_id } = model.material.shader {
                    if let Some(instance_buffer) = &model.instances.buffer {
                        render_pass.set_bind_group(1, &diffuse_textures[diffuse_texture_id].bind_group, &[]);
                        render_pass.set_vertex_buffer(0, mesh.vertices.buffer.slice(..));
                        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                        render_pass.draw(0..mesh.vertices.len as u32, 0..model.instances.raws.len() as u32);
                    }
                }
            }
        }
        render_pass.set_pipeline(&self.anim_diftex.render_pipeline);
        for mesh in meshes {
            for model in &mesh.models {
                if let crate::Shader::AnimatedDiffuseTexture { diffuse_texture_id } = model.material.shader {
                    render_pass.set_bind_group(1, &diffuse_textures[diffuse_texture_id].bind_group, &[]);
                    render_pass.set_vertex_buffer(0, mesh.vertices.buffer.slice(..));
                    render_pass.set_vertex_buffer(1, model.instances.buffer.as_ref().unwrap().slice(..));
                    let skins = model.instances.skins.as_ref().expect("Model has AnimatedDiffuseTexture shader but instances has no skins");
                    for (i, skin) in skins.iter().enumerate() {
                        render_pass.set_bind_group(2, &skin.bind_group, &[]);
                        render_pass.draw(0..mesh.vertices.len as u32, i as u32..(i as u32 +1));
                    }
                }
            }
        }
    }
}