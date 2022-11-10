use shared::{Context, glam::Vec2};
use crate::MyGame;

impl MyGame {
    pub fn poll_input(&mut self, engine: &mut dyn Context) {
        self.dir = Vec2::new(0.0, 0.0);
        self.dir.y = if engine.is_key_down(40) {-1.0} else if engine.is_key_down(36) {1.0} else {0.0};
        self.dir.x = if engine.is_key_down(18) {-1.0} else if engine.is_key_down(21) {1.0} else {0.0};

        
        if let Some(key) = engine.last_key_pressed() {
        //    println!("{}", key);
        }
    }
}