use std::rc::Rc;
use crate::{Material, Shader};

pub struct Mesh {
    pub vertices: compiler::Vertices,
    pub models: Vec<Model>,
    pub skin: Option<crate::MeshSkin>
}

pub struct Model {
    pub id: usize,
    pub mesh_id: usize,
    pub material: crate::Material,
    pub instances: crate::Instances
}

#[derive(Clone)]
pub struct ModelPTR {
    pub mesh_id: usize,
    pub model_id: usize
}

pub struct Scene {
    bgls: Rc<crate::Bgls>,
    device: Rc<wgpu::Device>,
    pub meshes: Vec<Mesh>
}

impl Scene {
    pub fn new(device: Rc<wgpu::Device>, bgls: Rc<crate::Bgls>) -> Self {
        Self {
            bgls,
            device,
            meshes: vec![]
        }
    }
    pub fn load_assets(&mut self) {
        let assets = crate::Assets::load(&self.device);
        for mesh in assets.meshes {
            self.meshes.push(Mesh {
                vertices: mesh.vertices,
                models: vec![],
                skin: None
            })
        }
    }
    pub fn add_model(&mut self, mesh_id: usize, shader: Shader) -> ModelPTR {
        let material = Material::new(&self.device, self.bgls.as_ref(), shader);
        match material.shader {
            Shader::Basic {..} => match self.meshes[mesh_id].vertices.vtype {
                compiler::VertexType::Normal => {},
                _ => panic!("invalid shader settings !. Shader::Basic shader can only be assigned to VertexType::Normal")
            },
            Shader::BasicDiffuseTexture {..} => match self.meshes[mesh_id].vertices.vtype {
                compiler::VertexType::NormalUV => {},
                _ => panic!("invalid shader settings !. Shader::BasicDiffuseTexture shader can only be assigned to VertexType::NormalUV")
            },
            Shader::AnimatedDiffuseTexture {..} => match self.meshes[mesh_id].vertices.vtype {
                compiler::VertexType::NormalUVSkin => {},
                _ => panic!("invalid shader settings !. Shader::AnimatedDiffuseTexture shader can only be assigned to VertexType::NormalUVSkin")
            }
        }
        let instances = crate::Instances::new();
        let model = Model {
            id: self.meshes[mesh_id].models.len(),
            mesh_id,
            material,
            instances
        };
        self.meshes[mesh_id].models.push(model);
        ModelPTR {
            mesh_id: mesh_id,
            model_id: self.meshes[mesh_id].models.len() - 1
        }
    }
    pub fn add_instance(&mut self, model: ModelPTR, position: [f32;3], rotation: [f32;2], size: f32) {
        let skin = match &self.meshes[model.mesh_id].skin {
            Some(mesh_skin) => Some(crate::InstanceSkin::new(&self.device, &self.bgls, &mesh_skin.joints)),
            None => None
        };
        self.meshes[model.mesh_id].models[model.model_id].instances.add(crate::InstanceRaw {
            position,
            rotation: 
            [
                rotation[0].cos(),
                rotation[0].sin(),
                rotation[1].cos(),
                rotation[1].sin()
            ],
            size
        }, skin)
    }
}