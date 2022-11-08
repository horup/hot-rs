use shared::Context;
use crate::MyGame;

impl MyGame {
    pub fn draw(&mut self, _ctx:&mut dyn Context) {
        /*let dt = canvas.ctx_mut().dt;
        let state = state_mut();
    
        state.flash_timer_sec -= dt;
        if state.flash_timer_sec < 0.0 {
            state.flash_timer_sec = 0.0;
        }
    
        let screen_size = canvas.screen_size();
        let scale = 2.0;
        let tex_size = canvas.texture_size(Textures::PokemonCard.into()) * scale;
        let margin = 8.0;
    
        canvas.draw_texture(DrawTextureParams {
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
            canvas.draw_rect(DrawRectParams {
                x:0.0,
                y:0.0,
                w:screen_size.x,
                h:screen_size.y,
                color:Color::new(1.0, 1.0, 1.0, a)
            });
           
        }*/
    }
}
