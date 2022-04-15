use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState},
    event_loop::{ControlFlow, EventLoop}
};

use crate::*;

pub struct Engine {
    event_loop: winit::event_loop::EventLoop<()>,
    game: Game,
    scripts: Vec<Box<dyn Script>>
}
impl Engine {
    pub fn new(settings: Settings) -> Engine {
        env_logger::init();
        let event_loop = EventLoop::new();
        let game = Game::new(settings, &event_loop);
        Self {
            event_loop,
            game,
            scripts: vec![]
        }
    }
    pub fn add_script(mut self, script: Box<dyn Script>) -> Self {
        self.scripts.push(script);
        self
    }
    pub fn start(self) {
        let event_loop = self.event_loop;
        let mut game = self.game;
        let mut scripts = self.scripts;

        for script in &mut scripts {
            script.setup(&mut game);
        }

        event_loop.run(move |event, _, control_flow| {
            if game.close_requested {
                *control_flow = ControlFlow::Exit;
            }else {
            match event {
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(key), ..
                        }, ..
                    } => {
                        for script in &mut scripts {
                            script.on_key_pressed(&mut game, key)
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(key), ..
                        }, ..
                    } => {
                        for script in &mut scripts {
                            script.on_key_released(&mut game, key)
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        if game.lock_cursor && game.window.on_focus {
                            if let Err(_) = game.window.window.set_cursor_position(winit::dpi::PhysicalPosition {
                                x: game.window.size_d2[0],
                                y: game.window.size_d2[1]
                            }) {};
                        }
                        for script in &mut scripts {
                            script.on_mouse_move(&mut game, position)
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        for script in &mut scripts {
                            if let winit::event::MouseScrollDelta::LineDelta(x, y) = delta {
                                script.on_mouse_wheel(&mut game, *x, *y)
                            }
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        for script in &mut scripts {
                            script.on_mouse_input(&mut game, state, button)
                        }
                    }
                    WindowEvent::CursorEntered {..} => game.set_focus(true),
                    WindowEvent::CursorLeft {..} => game.set_focus(false),
                    WindowEvent::Focused (focused) => game.set_focus(*focused),
                    WindowEvent::Resized (physical_size) => game.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => game.resize(**new_inner_size),
                    _ => {}
                }
                Event::RedrawRequested(window_id) if window_id == game.window.window.id() => {
                    for script in &mut scripts {
                        script.before_render(&mut game)
                    }
                    game.update(control_flow)
                }
                Event::MainEventsCleared => game.window.window.request_redraw(),
                _ => {}
            }
            }
        });
    }
}