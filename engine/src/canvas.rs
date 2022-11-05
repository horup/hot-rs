use context::{CanvasOrg, Context};
use macroquad::prelude::*;
use crate::MacroquadEngine;

impl CanvasOrg for MacroquadEngine {
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

    fn screen_size(&mut self) -> Vec2 {
        Vec2::new(screen_width(), screen_height())
    }

    fn texture_size(&mut self, texture:u32) -> Vec2 {
        if let Some(tex) = self.textures.get(&texture) {
            return Vec2::new(tex.width(), tex.height());
        }
        Vec2::default()
    }

    fn ctx_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    fn draw_rect(&mut self, params:context::DrawRectParams) {
        draw_rectangle(params.x, params.y, params.w, params.h, Color {
            r:params.color.r,
            g:params.color.g,
            b:params.color.b,
            a:params.color.a
        })
    }
}