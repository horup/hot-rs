use context::{Command, PlayerInput, glam::Vec2};
use macroquad::prelude::{is_key_pressed, KeyCode, is_key_down};

use crate::Engine;

impl Engine {
    pub fn input(&mut self) {
        if is_key_pressed(KeyCode::F1) {
            self.ctx.commands.push(Command::Restart);
        }
    
        let mut x = 0.0;
        let mut y = 0.0;
        let action = is_key_down(KeyCode::Space);
    
        if is_key_down(KeyCode::A) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            x += 1.0;
        }
        if is_key_down(KeyCode::W) {
            y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            y += 1.0;
        }
    
        self.ctx.player_input = PlayerInput { dir:Vec2::new(x, y), action: action };
    }
}