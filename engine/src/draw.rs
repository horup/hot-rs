use macroquad::prelude::*;
use crate::Engine;

impl Engine {
    pub fn perspective(&self) -> f32 {
        return 24.0 / 16.0;
    }

    pub fn scaler(&self) -> f32 {
        let zoom = self.ctx.state.camera.zoom;
        let w = screen_width();
        let h = screen_height();

        let w_a = zoom / w;
        let h_a = zoom / h;
        let mut a = w_a;

        if w < h {
            a = h_a;
        }

        return a;
    }

    
    pub fn to_world(&self, screen:Vec2) -> Vec2 {
        let w = screen_width();
        let h = screen_height();
       
        let center = self.ctx.state.camera.pos;
        let persp = self.perspective();
        let p  = Vec2::new(screen.x - w / 2.0, (screen.y - h / 2.0) * persp);
        let a = self.scaler();
        let p = p * a;
        let p = p + center;
        return p;
    }

    pub fn to_screen(&self, world:Vec2) -> Vec2 {
        let w = screen_width();
        let h = screen_height();
        let center = self.ctx.state.camera.pos;
        let a = self.scaler();
        
        let p = world - center;
        let p = p / a;
        let perspective = self.perspective();
        let p = Vec2::new(p.x + w / 2.0, p.y / perspective + h / 2.0);
        return p;
    }

    fn draw_cursor(&self) {
       /* let p = self.ctx.input.mouse_pos_world.floor();
        let p = self.to_screen(p);
        
        draw_circle(p.x, p.y, 16.0, WHITE);

        let p = self.ctx.input.mouse_pos_screen;
        draw_circle(p.x, p.y, 2.0, RED);*/

    }

    fn draw_edit_ui(&mut self) {
        let _space = 16.0;
       
    }

    fn draw_grid(&mut self) {
        let _o = self.to_world(Vec2::new(0.0, 0.0));
        let size = self.ctx.state.tilemap.size();
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

    pub fn draw_floor(&mut self) {
        let tilemap = &self.ctx.state.tilemap;
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
        }
    }


    pub fn draw_tex(&self, p:Vec2, tex:&Texture2D) {
        let p1 = self.to_screen(Vec2::new(p.x as f32,  p.y as f32));
        let p2 = self.to_screen(Vec2::new(p.x as f32 + 1.0, p.y as f32 + 1.0));
        let v = p2 - p1;
        let w = v.x;
        let h = v.y;

        let a = (1.5 / (tex.width() / tex.height())) - 1.0;
        let x = p1.x;
        let y = p1.y - a * h;
        
        draw_texture_ex(tex.clone(), x, y, WHITE, DrawTextureParams {
            dest_size:Some(Vec2::new(w,h + a * h)),
            ..Default::default()
        });
    }
    pub fn draw_map(&mut self) {
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
    
    pub fn draw(&mut self) {
        let dt = get_frame_time();
        if self.ctx.debug {
            self.draw_grid();
        }

        if self.ctx.edit_mode {
            self.draw_map();
        }

      //  self.draw_floor();
        for e in self.ctx.state.entities.iter() {
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

        self.draw_cursor();
        self.draw_edit_ui();
    }
}