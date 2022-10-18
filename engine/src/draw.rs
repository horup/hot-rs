
use macroquad::prelude::*;
use crate::Engine;

impl Engine {
    pub fn draw(&mut self) {
        let dt = get_frame_time();
        for e in self.context.state.entities.iter() {
            let tex = self.textures.get(&e.texture).unwrap();
            draw_texture(tex.clone(), e.x, e.y, WHITE);
        }

        self.flash_timer -= dt;

        if self.flash_timer < 0.0 {
            self.flash_timer = 0.0;
        } else {
            let a = self.flash_timer / self.flash_timer_start;
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(1.0, 1.0, 1.0, a));
        }
    }
}