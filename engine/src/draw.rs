use context::{Context};
use macroquad::{prelude::RED, shapes::draw_circle};
use crate::Engine;

impl Engine {
    pub fn draw(&mut self) {
        for e in self.context.state.entities.iter() {
            draw_circle(e.x, e.y, 10.0, RED);
        }
    }
}