use shared::*;
use crate::{MyGame, Textures};

impl MyGame {
    fn draw_hud_item(&self, ctx: &mut dyn Context, x: f32, y: f32, ico_size: f32, tex:u32, s:String) {
        ctx.draw_texture(DrawTextureParams {
            x:x - ico_size / 2.0,
            y:y - ico_size / 2.0,
            w:ico_size,
            h:ico_size,
            texture:tex,
            ..Default::default()
        });
        ctx.draw_string(DrawStringParams {
            str: s,
            x: x + ico_size / 4.0,
            y: y + ico_size / 8.0,
            font_height:ico_size / 2.0,
            ..Default::default()
        })
    }
    
    fn draw_hud(&self, ctx:&mut dyn Context) {
        let screen_size = ctx.screen_size();
        let scale = 14.0;
        let ico_size = screen_size.y / scale;
        let x = ico_size / 2.0;
        let y = ico_size / 2.0;
        let w = screen_size.x / 6.0;
        self.draw_hud_item(ctx, x, y, ico_size, Textures::PokemonCard.into(), format!("{:02}/{:02}", self.state.pokemon_cards.current, self.state.pokemon_cards.total));

        let inv = &self.state.inventory;
        let img = Textures::BlueKey;
        if inv.contains_key(&img) {
            self.draw_hud_item(ctx, x + w, y, ico_size, img.into(), "".into());
        }
        let img = Textures::GoldKey;
        if inv.contains_key(&img) {
            self.draw_hud_item(ctx, x + w + ico_size, y, ico_size, img.into(), "".into());
        }
    }

    fn draw_flash(&mut self, ctx:&mut dyn Context) {
        let dt = ctx.dt();
        let state = &mut self.state;
        state.flash_timer_sec -= dt;
        if state.flash_timer_sec < 0.0 {
            state.flash_timer_sec = 0.0;
        }
        let screen_size = ctx.screen_size();
        if state.flash_timer_sec > 0.0 && state.flash_timer_start > 0.0 {
            let a = state.flash_timer_sec / state.flash_timer_start;
            let a = a * state.flash_max;
            ctx.draw_rect(DrawRectParams {
                x:0.0,
                y:0.0,
                w:screen_size.x,
                h:screen_size.y,
                color:Color::new(1.0, 1.0, 1.0, a)
            });
        }
    }

    pub fn draw(&mut self, ctx:&mut dyn Context) {
        ctx.draw(&self.state.camera); 
        self.draw_flash(ctx);
        self.draw_hud(ctx);
    }
}
