use shared::*;
use crate::{MyGame, Textures};

impl MyGame {
    pub fn draw(&mut self, ctx:&mut dyn Context) {
        ctx.draw_world(&self.state.camera); 
        let dt = ctx.dt();
        let state = &mut self.state;
        state.flash_timer_sec -= dt;
        if state.flash_timer_sec < 0.0 {
            state.flash_timer_sec = 0.0;
        }

        let screen_size = ctx.screen_size();
        let scale = 2.0;
        let tex_size = ctx.texture_size(Textures::PokemonCard.into()) * scale;
        let margin = 8.0;

        ctx.draw_texture(DrawTextureParams {
            x:margin,
            y:margin,
            w:tex_size.x,
            h:tex_size.y,
            texture:Textures::PokemonCard.into(),
            ..Default::default()
        });

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
}
