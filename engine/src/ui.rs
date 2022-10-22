use context::Tool;
use macroquad::{prelude::*, ui::{widgets, root_ui}, hash};

use crate::Engine;


impl Engine {

    pub fn edit_ui(&mut self) {
        
        let margin = 0.0;
        let h = screen_height() - margin * 2.0;
        let w = 128.0;
        let label = match self.ctx.edit.tool {
            Tool::Tile => "Tiles",
            Tool::Entity => "Entities",
        };
        widgets::Window::new(hash!(), vec2(margin, margin), vec2(w, h))
        .label(label)
        .titlebar(true)
        .movable(false)
        .ui(&mut *root_ui(), |ui| {
            let textures:Vec<u32>;
            let mut selected:u32;

            match self.ctx.edit.tool {
                Tool::Tile => {
                    textures = self.ctx.edit.tiles.clone();
                    selected = self.ctx.edit.selected_tile;
                },

                Tool::Entity => {
                    textures = self.ctx.edit.entities.clone();
                    selected = self.ctx.edit.selected_entity;
                },
            }

            for tex_id in textures {
                if let Some(tex) = self.textures.get(&tex_id) {
                    let w = w/3.0;
                    let h = w * tex.height() / tex.width();
                    if ui.texture(tex.clone(), w, h) {
                        selected = tex_id.clone();
                    }
                    ui.same_line(0.0);
                    ui.label(None, if tex_id == selected {"Selected"} else {""});
                }
            }

            match self.ctx.edit.tool {
                Tool::Tile => {
                    self.ctx.edit.selected_tile = selected;
                },
                Tool::Entity => {
                    self.ctx.edit.selected_entity = selected;
                },
            }

            self.ctx.over_ui = ui.is_mouse_over(self.ctx.input.mouse_pos_screen);
        });
    }

    pub fn ui(&mut self) {
        self.ctx.over_ui = false;
        if self.ctx.edit_mode {
            self.edit_ui();
        }
    }
}