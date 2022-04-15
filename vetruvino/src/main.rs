use core::*;

struct MainScript {}
impl Script for MainScript {
    fn setup(&mut self, game: &mut Game) {
        game.load_assets();
        game.add_model(0, Shader::Basic { color: [1.0, 1.0, 1.0] });
        game.add_instance(0, 0, [0.0, 0.0, 0.0], [0.0, 0.0], 0.05);
        game.gui.add(Text {
            text: "Hello World !".to_string(),
            font_size: 25,
            .. Default::default()
        })
    }
    fn on_key_pressed(&mut self, game: &mut Game, key: &Key) {
        match key {
            Key::Escape => game.close(),
            _ => {}
        }
    }
}

fn main() {
    Engine::new(Settings {
        window: core::WindowSettings {
            title: "Vetruvino",
            fullscreen: Fullscreen::None,
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