use crate::{Game, Key};

pub struct FlyingCamera {
    pub move_with_mouse_key: bool,
    pub mouse_key: winit::event::MouseButton,
    pub mouse_key_pressed: bool,
    pub cursor_position: winit::dpi::PhysicalPosition<f64>,
    pub last_cursor_position: winit::dpi::PhysicalPosition<f64>,
    pub keys: [bool;8],
    pub acc: f32,
    pub vel: [f32;3],
    pub max_vel_normal: f32,
    pub max_vel_fast: f32,
    pub max_vel_slow: f32,
    pub ang: [f32;2],
    pub ang_target: [f32;2],
    pub ang_acc: f32,
    pub cursor_sens: f32
}
impl Default for FlyingCamera {
    fn default() -> Self {
        Self {
            move_with_mouse_key: true,
            mouse_key: winit::event::MouseButton::Right,
            mouse_key_pressed: false,
            cursor_position: winit::dpi::PhysicalPosition{x: 0.0, y: 0.0},
            last_cursor_position: winit::dpi::PhysicalPosition{x: 0.0, y: 0.0},
            keys: [false, false, false, false, false, false, false, false],
            acc: 0.1,
            vel: [0.0, 0.0, 0.0],
            max_vel_normal: 0.1,
            max_vel_fast: 0.2,
            max_vel_slow: 0.02,
            ang: [3.1415926, 0.0],
            ang_target: [3.1415926, 0.0],
            ang_acc: 0.2,
            cursor_sens: 0.001
        }
    }
}
impl FlyingCamera {
    pub fn can_move(&self) -> bool {
        !self.move_with_mouse_key || self.mouse_key_pressed
    }
}

impl crate::Script for FlyingCamera {
    fn setup(&mut self, game: &mut Game) {
        game.camera.set_position([0.0, 5.0, 10.0]);
    }
    fn on_mouse_input(&mut self, game: &mut Game, state: &winit::event::ElementState, button: &winit::event::MouseButton) {
        if !self.move_with_mouse_key {return}
        let state = match state {
            winit::event::ElementState::Pressed => {
                self.last_cursor_position = self.cursor_position;
                if let Err(_) = game.window.window.set_cursor_position(winit::dpi::PhysicalPosition {
                    x: game.window.size_d2[0],
                    y: game.window.size_d2[1]
                }) {};
                true
            },
            winit::event::ElementState::Released => {
                if let Err(_) = game.window.window.set_cursor_position(self.last_cursor_position) {};
                for key in &mut self.keys {
                    *key = false;
                }
                false
            }
        };
        match button {
            winit::event::MouseButton::Left => if let winit::event::MouseButton::Left = self.mouse_key {
                self.mouse_key_pressed = state
            }
            winit::event::MouseButton::Middle => if let winit::event::MouseButton::Middle = self.mouse_key {
                self.mouse_key_pressed = state
            }
            winit::event::MouseButton::Other(btn_id) => if let winit::event::MouseButton::Other(c_btn_id) = self.mouse_key {
                if *btn_id == c_btn_id { self.mouse_key_pressed = state }
            }
            winit::event::MouseButton::Right => if let winit::event::MouseButton::Right = self.mouse_key {
                self.mouse_key_pressed = state
            }
        }
        game.window.window.set_cursor_visible(!state);
    }
    fn on_key_pressed(&mut self, _: &mut Game, key: &Key) {
        if !self.can_move() {return}
        match key {
            Key::W => self.keys[0] = true,
            Key::S => self.keys[1] = true,
            Key::A => self.keys[2] = true,
            Key::D => self.keys[3] = true,
            Key::E => self.keys[4] = true,
            Key::Q => self.keys[5] = true,
            Key::Space => self.keys[4] = true,
            Key::LAlt => self.keys[5] = true,
            Key::LShift => self.keys[6] = true,
            Key::LControl => self.keys[7] = true,
            _ => {}
        }
    }
    fn on_key_released(&mut self, _: &mut Game, key: &Key) {
        if !self.can_move() {return}
        match key {
            Key::W => self.keys[0] = false,
            Key::S => self.keys[1] = false,
            Key::A => self.keys[2] = false,
            Key::D => self.keys[3] = false,
            Key::E => self.keys[4] = false,
            Key::Q => self.keys[5] = false,
            Key::Space => self.keys[4] = false,
            Key::LAlt => self.keys[5] = false,
            Key::LShift => self.keys[6] = false,
            Key::LControl => self.keys[7] = false,
            _ => {}
        }
    }
    fn on_mouse_move(&mut self, game: &mut Game, position: &winit::dpi::PhysicalPosition<f64>) {
        self.cursor_position = *position;
        if !self.can_move() {return}
        self.ang_target[0] += (position.x as f32 - game.window.position[0] as f32 - game.window.size_d2[0] as f32) * self.cursor_sens;
        self.ang_target[1] += (position.y as f32 - game.window.position[1] as f32 - game.window.size_d2[1] as f32) * -self.cursor_sens;
    }
    fn on_mouse_wheel(&mut self, game: &mut Game, _: f32, y: f32){
        if !self.can_move() {return}
        game.camera.add_fov(-y)
    }
    fn before_render(&mut self, game: &mut Game) {
        game.lock_cursor = self.can_move();
        let max_vel =
            if self.keys[6] { self.max_vel_fast } else
            if self.keys[7] { self.max_vel_slow } else {
                self.max_vel_normal };
        let vel_target = [
            if self.keys[0] { max_vel } else if self.keys[1] { -max_vel } else { 0.0 },
            if self.keys[2] { max_vel } else if self.keys[3] { -max_vel } else { 0.0 },
            if self.keys[4] { max_vel } else if self.keys[5] { -max_vel } else { 0.0 }
        ];
        self.ang = [
            self.ang[0] + ((self.ang_target[0] - self.ang[0]) * self.ang_acc),
            self.ang[1] + ((self.ang_target[1] - self.ang[1]) * self.ang_acc)
        ];
        self.vel = [
            self.vel[0] + ((vel_target[1] - self.vel[0]) * self.acc),
            self.vel[1] + ((vel_target[2] - self.vel[1]) * self.acc),
            self.vel[2] + ((vel_target[0] - self.vel[2]) * self.acc)
        ];
        let vel = math::vec3::rotate_y(self.vel, self.ang[0]);
        game.camera.translate(vel);
        let position = game.camera.get_position();
        let mut target = math::vec3::rotate_xy([0.0, 0.0, 1.0], self.ang[1], self.ang[0]);
        target[0] += position[0];
        target[1] += position[1];
        target[2] += position[2];
        game.camera.set_target(target);
    }
}