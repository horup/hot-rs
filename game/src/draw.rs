use crate::{Images, MyGame};
use shared::*;

impl MyGame {
    fn draw_hud_item(
        &self,
        ctx: &mut dyn Context,
        x: f32,
        y: f32,
        ico_size: f32,
        tex: u32,
        s: String,
    ) {
        ctx.draw_img(DrawImgParams {
            x: x - ico_size / 2.0,
            y: y - ico_size / 2.0,
            w: ico_size,
            h: ico_size,
            img: tex,
            ..Default::default()
        });
        ctx.draw_string(DrawStringParams {
            str: s,
            x: x + ico_size / 4.0,
            y: y + ico_size / 8.0,
            font_height: ico_size / 2.0,
            ..Default::default()
        })
    }

    fn draw_hud(&self, ctx: &mut dyn Context) {
        let screen_size = ctx.screen_size();
        let scale = 14.0;
        let ico_size = screen_size.y / scale;
        let x = ico_size / 2.0;
        let y = ico_size / 2.0;
        let w = screen_size.x / 6.0;
        self.draw_hud_item(
            ctx,
            x,
            y,
            ico_size,
            Images::PokemonCard.into(),
            format!(
                "{:02}/{:02}",
                self.state.pokemon_cards.current, self.state.pokemon_cards.total
            ),
        );

        let inv = &self.state.inventory;
        let img = Images::BlueKey;
        if inv.contains_key(&img) {
            self.draw_hud_item(ctx, x + w, y, ico_size, img.into(), "".into());
        }
        let img = Images::GoldKey;
        if inv.contains_key(&img) {
            self.draw_hud_item(ctx, x + w + ico_size, y, ico_size, img.into(), "".into());
        }
    }

    fn draw_flash(&mut self, ctx: &mut dyn Context) {
        let dt = ctx.dt();
        let state = &mut self.state;
        state.flash.flash_timer_sec -= dt;
        if state.flash.flash_timer_sec < 0.0 {
            state.flash.flash_timer_sec = 0.0;
        }
        let screen_size = ctx.screen_size();
        if state.flash.flash_timer_sec > 0.0 && state.flash.flash_timer_start > 0.0 {
            let a = state.flash.flash_timer_sec / state.flash.flash_timer_start;
            let a = a * state.flash.flash_max;
            ctx.draw_rect(DrawRectParams {
                rect:Rect2 { x: 0.0, y: 0.0, w: screen_size.x, h: screen_size.y },
                color: Color::new(1.0, 1.0, 1.0, a),
            });
        }
    }

    pub fn draw(&mut self, ctx: &mut dyn Context) {
        if self.state.chosen_character.is_none() {
            self.draw_character_selector(ctx);
            return;
        } 

        if self.state.won {
            self.draw_won(ctx);
            return;
        }
        
        ctx.draw(
            &self.state.camera,
            &self.state,
            DrawParams {
                debug_entity: false,
            },
        );
        self.draw_flash(ctx);
        self.draw_hud(ctx);
    }

    pub fn draw_won(&mut self, ctx:&mut dyn Context) {
        if self.state.won == false {
            return;
        }

        let screen = ctx.screen_size();
        let font_height = screen.x / 16.0;
        ctx.draw_string(DrawStringParams {
            str: "You Escaped!!!".into(),
            x: screen.x / 2.0,
            y: screen.y / 2.0,
            font_height,
            color: WHITE,
            alignment_horizontal: Alignment::Center,
        });
    }

    pub fn draw_character_selector(&mut self, ctx:&mut dyn Context) {
        let screen = ctx.screen_size();
        let font_height = screen.x / 16.0;
        let y = screen.y / 6.0;
        ctx.draw_string(DrawStringParams {
            str: "Please select your character!".into(),
            x: screen.x / 2.0,
            y: y - font_height,
            font_height,
            color: WHITE,
            alignment_horizontal: Alignment::Center,
        });

        let sx = screen.x / 6.0;
    
        let chars = [
            (screen.x / 2.0 - sx, Images::William, "William"), 
            (screen.x / 2.0 + sx, Images::Viktor, "Viktor")
        ];
       
        let mouse_pos = ctx.mouse_pos();
        for (x, img, name) in chars {
            let w = sx * 1.5;
            let h = w * 1.5;
            

            let r = Rect2::new(x - w /2.0, y, w, h);
            let mut color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.25 };
            if r.contains(&mouse_pos) {
                color.a = 0.5;

                if ctx.mouse_button_pressed(1) {
                    self.state.chosen_character = Some(img);
                    self.start(img);
                }
            }
            ctx.draw_rect(DrawRectParams {
                rect: r,
                color: color,
            });

            ctx.draw_img(DrawImgParams {
                img: img.into(),
                x:r.x,
                y:r.y - h / 3.0,
                w,
                h:w * 2.0,
                ..Default::default()
            });

            ctx.draw_string(DrawStringParams {
                str: name.into(),
                x,
                y: y + h + font_height,
                font_height: font_height,
                color:WHITE,
                alignment_horizontal: Alignment::Center,
            });
        }

        

       /* ctx.draw_img(DrawImgParams {
            img:Images::William.into(),
            x:x - s / 2.0,
            y:y - s,
            w:s,
            h:s * 2.0,
            ..Default::default()
        });

        let x = sx * 4.0;

        ctx.draw_img(DrawImgParams {
            img:Images::Viktor.into(),
            x:x - s / 2.0,
            y:y - s,
            w:s,
            h:s * 2.0,
            ..Default::default()
        });*/


       // ctx.draw_img(params)


    }
}
