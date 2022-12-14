use macroquad::prelude::*;
use crate::Engine;

impl Engine {
    pub fn active_camera(&self) -> &shared::Camera {
        if self.edit_mode {
            return &self.edit_camera;
        }

        &self.game_camera
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

    pub fn cell_size_screen(&self) -> Vec2 {
        let p1 = self.to_screen(Vec2::new(0.0, 0.0));
        let p2 = self.to_screen(Vec2::new(1.0, 1.0));
        p2 - p1
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

    fn _draw_grid(&self) {
        let _o = self.to_world(Vec2::new(0.0, 0.0));
        let size = self.map.grid.size();
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

    pub fn draw_sprite(&self, p:Vec3, tex:&Texture2D, flip_x:bool, flip_y:bool, color:Color) {
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
        let y = y + -p.z * h;

        draw_texture_ex(*tex, x, y, color, DrawTextureParams {
            dest_size:Some(Vec2::new(dw,dh)),
            flip_x,
            flip_y,
            ..Default::default()
        });
    }

    pub fn draw_tex(&self, p:Vec2, tex:&Texture2D, color:Color) {
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

        draw_texture_ex(*tex, x, y, color, DrawTextureParams {
            dest_size:Some(Vec2::new(dw,dh)),
            ..Default::default()
        });
    }
    pub fn draw_map(&self) {
        let map = &self.map;
        for y in 0..map.grid.size() {
            for x in 0..map.grid.size() {
                let p1 = self.to_screen(Vec2::new(x as f32,  y as f32));
                let p2 = self.to_screen(Vec2::new(x as f32 + 1.0, y as f32 + 1.0));
                let _w = p2.x - p1.x;
                let _h = p2.y - p1.y;

                if let Some(cell) = map.grid.get(x as i32, y as i32) {
                    if let Some(tile) = cell.tile {
                        if let Some(tex) = self.textures.get(&tile) {
                            self.draw_tex(Vec2::new(x as f32, y as f32), tex, WHITE);
                        }
                    }

                    if cell.blocks {
                        draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, RED);
                        draw_line(p1.x, p2.y, p2.x, p1.y, 1.0, RED);
                    }

                    if let Some(entity) = cell.entity {
                        if let Some(tex) = self.textures.get(&entity) {
                            self.draw_tex(Vec2::new(x as f32, y as f32), tex, WHITE);
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
       txt!(&format!("Mouse Pos: {:.2},{:.2}", self.edit_input.mouse_pos_world.x, self.edit_input.mouse_pos_world.y));
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
}