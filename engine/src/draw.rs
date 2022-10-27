use context::EntityKey;
use macroquad::prelude::*;
use crate::Engine;

impl Engine {
    pub fn active_camera(&self) -> &context::Camera {
        if self.ctx.edit_mode {
            return &self.ctx.edit_camera;
        }

        &self.ctx.game_camera
    }
    pub fn perspective(&self) -> f32 {
        24.0 / 16.0
    }

    pub fn scaler(&self) -> f32 {
        let zoom = self.active_camera().zoom;
        let w = screen_width();
        let h = screen_height();

        let w_a = zoom / w;
        let h_a = zoom / h;
        let mut a = w_a;

        if w < h {
            a = h_a;
        }

        a
    }

    
    pub fn to_world(&self, screen:Vec2) -> Vec2 {
        let w = screen_width();
        let h = screen_height();
       
        let center = self.active_camera().pos;
        let persp = self.perspective();
        let p  = Vec2::new(screen.x - w / 2.0, (screen.y - h / 2.0) * persp);
        let a = self.scaler();
        let p = p * a;
        
        p + center
    }

    pub fn to_screen(&self, world:Vec2) -> Vec2 {
        let w = screen_width();
        let h = screen_height();
        let center = self.active_camera().pos;
        let a = self.scaler();
        
        let p = world - center;
        let p = p / a;
        let perspective = self.perspective();
        
        Vec2::new(p.x + w / 2.0, p.y / perspective + h / 2.0)
    }

    fn draw_grid(&self) {
        let _o = self.to_world(Vec2::new(0.0, 0.0));
        let size = self.ctx.tilemap.size();
        for x in 0..(size+1) {
            let x = x as f32;
            let p1 = self.to_screen(Vec2::new(x, 0.0));
            let p2 = self.to_screen(Vec2::new(x, size as f32));
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, WHITE);
        }
        for y in 0..size {
            let y = y as f32;
            let p1 = self.to_screen(Vec2::new(0.0, y));
            let p2 = self.to_screen(Vec2::new(size as f32, y));
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, WHITE);
        }
    }

    pub fn draw_floor(&self) {
        let tilemap = &self.ctx.tilemap;
        for y in 0..tilemap.size() {
            for x in 0..tilemap.size() {
                let p1 = self.to_screen(Vec2::new(x as f32,  y as f32));
                let p2 = self.to_screen(Vec2::new(x as f32 + 1.0, y as f32 + 1.0));
                let w = p2.x - p1.x;
                let h = p2.y - p1.y;

                if let Some(tile) = tilemap.get(x as i32, y as i32) {
                    if let Some(tex) = self.textures.get(&tile.texture) {
                        let a = (1.5 / (tex.width() / tex.height())) - 1.0;
                        let x = p1.x;
                        let y = p1.y - a * h;
                        
                        
                        draw_texture_ex(*tex, x, y, WHITE, DrawTextureParams {
                            dest_size:Some(Vec2::new(w,h + a * h)),
                            ..Default::default()
                        });
                        //draw_rectangle(p1.x, p1.y, w, h, RED);
                    }
                }

            }
        }
    }


    pub fn draw_sprite(&self, p:Vec2, tex:&Texture2D, flip_x:bool, flip_y:bool) {
        let p1 = self.to_screen(Vec2::new(p.x,  p.y));
        let p2 = self.to_screen(Vec2::new(p.x + 1.0, p.y + 1.0));
        let v = p2 - p1;
        let w = v.x;
        let h = v.y;

        let a = (1.5 / (tex.width() / tex.height())) - 1.0;
        let x = p1.x;
        let y = p1.y - a * h;
        
        let dw = w;
        let dh = h + a * h;

        let x = x - w/2.0;
        let y = y - h/2.0;

        draw_texture_ex(*tex, x, y, WHITE, DrawTextureParams {
            dest_size:Some(Vec2::new(dw,dh)),
            flip_x,
            flip_y,
            ..Default::default()
        });
    }

    pub fn draw_tex(&self, p:Vec2, tex:&Texture2D) {
        let p1 = self.to_screen(Vec2::new(p.x,  p.y));
        let p2 = self.to_screen(Vec2::new(p.x + 1.0, p.y + 1.0));
        let v = p2 - p1;
        let w = v.x;
        let h = v.y;

        let a = (1.5 / (tex.width() / tex.height())) - 1.0;
        let x = p1.x;
        let y = p1.y - a * h;
        
        let dw = w;
        let dh = h + a * h;

        draw_texture_ex(*tex, x, y, WHITE, DrawTextureParams {
            dest_size:Some(Vec2::new(dw,dh)),
            ..Default::default()
        });
    }
    pub fn draw_map(&self) {
        let map = &self.ctx.map;
        for y in 0..map.grid.size() {
            for x in 0..map.grid.size() {
                let p1 = self.to_screen(Vec2::new(x as f32,  y as f32));
                let p2 = self.to_screen(Vec2::new(x as f32 + 1.0, y as f32 + 1.0));
                let _w = p2.x - p1.x;
                let _h = p2.y - p1.y;

                if let Some(cell) = map.grid.get(x as i32, y as i32) {
                    if let Some(tile) = cell.tile {
                        if let Some(tex) = self.textures.get(&tile) {
                            self.draw_tex(Vec2::new(x as f32, y as f32), tex);
                        }
                    }

                    if cell.blocks {
                        draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, RED);
                        draw_line(p1.x, p2.y, p2.x, p1.y, 1.0, RED);
                    }

                    if let Some(entity) = cell.entity {
                        if let Some(tex) = self.textures.get(&entity) {

                            self.draw_tex(Vec2::new(x as f32, y as f32), tex);
                        }
                    }
                    
                }
            }
        }

        for y in 0..map.grid.size() {
            for x in 0..map.grid.size() {
                let p1 = self.to_screen(Vec2::new(x as f32,  y as f32));
                let p2 = self.to_screen(Vec2::new(x as f32 + 1.0, y as f32 + 1.0));
                let _w = p2.x - p1.x;
                let _h = p2.y - p1.y;
                if let Some(cell) = map.grid.get(x as i32, y as i32) {
                    if cell.blocks {
                        draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, RED);
                        draw_line(p1.x, p2.y, p2.x, p1.y, 1.0, RED);
                    }
                }
            }
        }
       /* let tilemap = &self.ctx.state.tilemap;
        for y in 0..tilemap.size() {
            for x in 0..tilemap.size() {
                let p1 = self.to_screen(Vec2::new(x as f32,  y as f32));
                let p2 = self.to_screen(Vec2::new(x as f32 + 1.0, y as f32 + 1.0));
                let w = p2.x - p1.x;
                let h = p2.y - p1.y;

                if let Some(tile) = tilemap.get(x as i32, y as i32) {
                    if let Some(tex) = self.textures.get(&tile.texture) {
                        let a = (1.5 / (tex.width() / tex.height())) - 1.0;
                        let x = p1.x;
                        let y = p1.y - a * h;
                        
                        
                        draw_texture_ex(tex.clone(), x, y, WHITE, DrawTextureParams {
                            dest_size:Some(Vec2::new(w,h + a * h)),
                            ..Default::default()
                        });
                        //draw_rectangle(p1.x, p1.y, w, h, RED);
                    }
                }

            }
        }*/
    }
    
    pub fn draw_debug(&mut self) {
        let mut y = 0.0;
        macro_rules! txt {
            ($s:expr) => {
                let size = 16.0;
                let m = measure_text($s, None, size as u16, 1.0);
                y += size;
                let x = screen_width() - m.width - size / 2.0;
                draw_text($s, x, y, size, WHITE);
            };
        }

       txt!(&format!("FPS: {:?}", get_fps()));
       txt!(&format!("Mouse Pos: {:.2},{:.2}", self.ctx.input.mouse_pos_world.x, self.ctx.input.mouse_pos_world.y));
       txt!(&format!("Zoom: {:.2}", self.active_camera().zoom));
    }

    pub fn draw_edit_mode(&self) {
        self.draw_map();
    }

    pub fn bounds(&self) -> Rect {
        let p1 = self.to_world(Vec2::new(0.0, 0.0));
        let p2 = self.to_world(Vec2::new(screen_width(), screen_height()));
        let v = p2 - p1;
        Rect::new(p1.x, p1.y, v.x, v.y)
    }

    pub fn draw_game_mode(&mut self) {
        self.flash_timer -= get_frame_time();
        if self.flash_timer < 0.0 {
            self.flash_timer = 0.0;
        } else {
            let a = self.flash_timer / self.flash_timer_start;
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(1.0, 1.0, 1.0, a));
        }

        let bounds = self.bounds();
        let margin = 3.0;
        let bounds = Rect {
            x: bounds.x - margin,
            y: bounds.y - margin,
            w: bounds.w + margin * 2.0,
            h: bounds.h + margin * 2.0,
        };
        let mut visible_set:Vec<EntityKey> = Vec::with_capacity(self.ctx.entities.len());

        for (key, e) in self.ctx.entities.iter() {
            if bounds.contains(e.pos.truncate()) {
                visible_set.push(key);
            }
        }

        visible_set.sort_by(|a, b| {
            if let (Some(a), Some(b)) = (self.ctx.entities.get(*a), self.ctx.entities.get(*b)){
                if a.pos.y < b.pos.y {
                    return std::cmp::Ordering::Less;
                } else if a.pos.y > b.pos.y {
                    return std::cmp::Ordering::Greater;
                }
            }

            std::cmp::Ordering::Equal
        });

        for cell_y in bounds.top() as i32 .. bounds.bottom() as i32 {
            for cell_x in bounds.left() as i32 .. bounds.right() as i32 {
                if let Some(cell) = self.ctx.map.grid.get(cell_x, cell_y) {
                    if let Some(tile) = cell.tile {
                        if let Some(tex) = self.textures.get(&tile) {
                            self.draw_tex(Vec2::new(cell_x as f32, cell_y as f32), tex);
                        }
                    }
                }
                for key in visible_set.iter() {
                    if let Some(e) = self.ctx.entities.get(*key) {
                        if e.pos.y as i32 == cell_y {
                            if let Some(tex) = self.textures.get(&e.texture) {
                                self.draw_sprite(e.pos.truncate(), tex, false, false);
                            }
                        }
                    }
                }
            }
        }

        if self.ctx.debug {
            for (_, e) in self.ctx.entities.iter() {
                let p = self.to_screen(e.pos.truncate());
                draw_circle(p.x, p.y, 2.0, RED);
            }
        }
    }

    pub fn draw(&mut self) {
        if self.ctx.debug {
            self.draw_grid();
        }

        if self.ctx.edit_mode {
            self.draw_edit_mode();
        } else {
            self.draw_game_mode();
        }

        if self.ctx.debug {
            self.draw_debug();
        }
    }
}