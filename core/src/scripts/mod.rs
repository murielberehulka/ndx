use super::{Game, Key};

mod flying_camera;
pub use flying_camera::FlyingCamera;

pub trait Script {
    fn setup(&mut self, _: &mut Game) {}
    fn before_render(&mut self, _: &mut Game) {}
    fn on_key_pressed(&mut self, _: &mut Game, _: &Key) {}
    fn on_key_released(&mut self, _: &mut Game, _: &Key) {}
    fn on_mouse_move(&mut self, _: &mut Game, _: &winit::dpi::PhysicalPosition<f64>) {}
    fn on_mouse_wheel(&mut self, _: &mut Game, _: f32, _: f32) {}
    fn on_mouse_input(&mut self, _: &mut Game, _: &winit::event::ElementState, _: &winit::event::MouseButton) {}
}