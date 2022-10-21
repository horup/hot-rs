use context::Tool;
use macroquad::{prelude::*, ui::{widgets, root_ui}, hash};

use crate::Engine;


impl Engine {
    pub fn ui(&mut self) {
        let margin = 0.0;
        let h = screen_height() - margin * 2.0;
        let w = 128.0;
        widgets::Window::new(hash!(), vec2(margin, margin), vec2(w, h))
        .label("Toolbox")
        .titlebar(true)
        .movable(false)
        .ui(&mut *root_ui(), |ui| {
            let mut tile = self.ctx.edit.tool == Tool::Tile;
            ui.checkbox(hash!(), "Tile", &mut tile);
            if tile {
                self.ctx.edit.tool = Tool::Tile;
            }

            let mut entity = self.ctx.edit.tool == Tool::Entity;
            ui.checkbox(hash!(), "Entity", &mut entity);
            if entity {
                self.ctx.edit.tool = Tool::Entity;
            }
            
            for (handle, tex) in self.textures.iter() {
                let w = w/3.0;
                let h = w * tex.height() / tex.width();
                let mut selected = 0;
                match self.ctx.edit.tool {
                    Tool::Tile => {
                        selected = self.ctx.edit.tile_texture;
                    },
                    Tool::Entity => {
                        selected = self.ctx.edit.entity_texture;
                    },
                }
                if selected == *handle {
                    ui.label(None, "Selected");
                } else {
                    ui.label(None, "");
                }
                if ui.texture(tex.clone(), w, h) {
                    selected = *handle;
                }

                match self.ctx.edit.tool {
                    Tool::Tile => {
                        self.ctx.edit.tile_texture = selected;
                    },
                    Tool::Entity => {
                        self.ctx.edit.entity_texture = selected;
                    },
                }
            }

            self.ctx.over_ui = ui.is_mouse_over(self.ctx.input.mouse_pos_screen);
        });
    }
}