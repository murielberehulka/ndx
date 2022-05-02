pub use winit::event::VirtualKeyCode as Key;
pub use winit::event::MouseButton;
pub use texture::*;
pub use math::*;
pub use compiler;
pub use compiler::Assets;

pub const MAX_IBMS: usize = utils::env_u32!("MAX_IBMS") as usize;

pub mod adapter;

pub mod scripts;
pub use scripts::Script;
pub mod bgls;
pub use bgls::Bgls;
pub mod material;
pub use material::Material;
pub mod shaders;
pub use shaders::{Shaders, Shader};
pub mod instance;
pub use instance::{Instances, InstanceRaw};

mod game;
pub use game::*;
mod scene;
pub use scene::*;
mod settings;
pub use settings::*;
mod skin;
pub use skin::*;
mod window;
pub use window::Window;
mod surface;
pub use surface::Surface;
mod engine;
pub use engine::Engine;
mod animation;
pub use animation::Animation;
mod camera;
pub use camera::{Camera, CameraRaw};