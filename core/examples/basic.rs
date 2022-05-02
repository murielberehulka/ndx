use core::{Game, Script, Shader, Key};

struct MainScript {}
impl Script for MainScript {
    fn setup(&mut self, game: &mut Game) {
        game.scene.load_assets();
        game.load_diffuse_texture("assets/meshes/mutant/textures/Mutant_diffuse.png");
        let model = game.scene.add_model(0, Shader::BasicDiffuseTexture { diffuse_texture_id: 0 });
        game.scene.add_instance(model, [0.0, 0.0, 0.0], [0.0, 0.0], 1.0);
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