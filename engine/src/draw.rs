use macroquad::prelude::*;
use crate::Engine;

impl Engine {
    fn screen_pos(&self, world_pos:Vec2) -> Vec2 {
        let zoom = self.context.state.camera.zoom;
        let center = self.context.state.camera.pos;
        let w = screen_width();
        let h = screen_height();

        let w_a = zoom / w;
        let h_a = zoom / h;
        let mut a = w_a;

        if w < h {
            a = h_a;
        }
        
        let p = world_pos - center;
        let p = p / a;
        let perspective = 24.0 / 16.0;
        let p = Vec2::new(p.x + w / 2.0, p.y / perspective + h / 2.0);
        return p;
    }

    fn draw_grid(&mut self) {
        let size = 64;
        for x in 0..(size+1) {
            let x = x as f32;
            let p1 = self.screen_pos(Vec2::new(x, 0.0));
            let p2 = self.screen_pos(Vec2::new(x, size as f32));
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, WHITE);
        }
        for y in 0..size {
            let y = y as f32;
            let p1 = self.screen_pos(Vec2::new(0.0, y));
            let p2 = self.screen_pos(Vec2::new(size as f32, y));
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, WHITE);
        }
    }
    pub fn draw(&mut self) {
        if self.context.debug {
            self.draw_grid();
        }
        let dt = get_frame_time();
        for e in self.context.state.entities.iter() {
            let tex = self.textures.get(&e.texture).unwrap();
            let w = tex.width();
            let h = tex.height();
            //draw_texture(tex.clone(), e.x, e.y, WHITE);
            let _a = w/h;
            let _x = e.x;
            let _y = e.y;
          /*  draw_texture_ex(tex, x, y, WHITE, 
                DrawTextureParams { 
                    dest_size: (), 
                    source: (), 
                    rotation: (), 
                    flip_x: (),
                    flip_y: (), 
                    pivot: () });*/
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