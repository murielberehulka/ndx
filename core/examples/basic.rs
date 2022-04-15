use core::{Game, Script, Shader, Key};

struct MainScript {}
impl Script for MainScript {
    fn setup(&mut self, game: &mut Game) {
        game.load_assets();
        game.load_diffuse_texture("assets/mutant/textures/Mutant_diffuse.png");
        game.add_model(1, Shader::AnimatedDiffuseTexture { diffuse_texture_id: 0 });
        game.add_instance(1, 0, [0.0, 0.0, 0.0], [0.0, 0.0], 0.05);
    }
    fn on_key_pressed(&mut self, game: &mut Game, key: &Key) {
        match key {
            Key::Escape => game.close(),
            _ => {}
        }
    }
}

fn main() {
    core::Engine::new(core::Settings {
        window: core::WindowSettings {
            title: "Example Basic",
            fullscreen: core::Fullscreen::None,
            maximized: false,
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .add_script(Box::new(MainScript {}))
    .add_script(Box::new(core::scripts::FlyingCamera::default()))
    .start()
}