use std::{fs::File, io::{Write, Read}};

use shared::{Context, glam::Vec2};
use crate::MyGame;

impl MyGame {
    pub fn poll_input(&mut self, engine: &mut dyn Context) {
        self.dir = Vec2::new(0.0, 0.0);
        self.dir.y = if engine.is_key_down(40) {-1.0} else if engine.is_key_down(36) {1.0} else {0.0};
        self.dir.x = if engine.is_key_down(18) {-1.0} else if engine.is_key_down(21) {1.0} else {0.0};

        let quick_save_path = "quicksave.sav";
        if let Some(key) = engine.last_key_pressed() {
            if key == 73 {
                let state = engine.serialize();
                println!("{:?}", state.len());
                let mut quick_save_file = File::create(quick_save_path).unwrap();
                quick_save_file.write_all(&state).unwrap();
                println!("saved state");
            } else if key == 74 {
                if let Ok(mut f) = File::open(quick_save_path) {
                    let mut buf = Vec::new();
                    if let Ok(_) = f.read_to_end(&mut buf) {
                        engine.deserialize(&buf);
                    }
                }
                println!("loaded state");
            }
            // println!("{}", _key);
        }
    }
}