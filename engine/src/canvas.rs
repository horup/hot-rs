use context::Canvas;
use macroquad::prelude::*;
use crate::Engine;

impl Canvas for Engine {
    fn draw_string(&mut self, p:context::DrawStringParams) {
        draw_text_ex(&p.str, p.x, p.y, TextParams {
            ..Default::default()
        });
    }

    fn draw_texture(&mut self, p:context::DrawTextureParams) {
        if let Some(tex) = self.textures.get(&p.texture) {
            draw_texture_ex(tex.clone(), p.x, p.y, WHITE, DrawTextureParams {
                dest_size:Some(Vec2::new(p.w, p.h)),
                ..Default::default()
            });
        }
    }
}