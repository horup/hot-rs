use context::Tool;
use macroquad::{prelude::*, ui::{widgets, root_ui}, hash};

use crate::Engine;


impl Engine {
    pub fn ui(&mut self) {
        let margin = 16.0;
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
            let size = 24.0;
            if ui.texture(self.textures.get(&1).unwrap().clone(), size, size) {

            }
            if ui.texture(self.textures.get(&1).unwrap().clone(), size, size) {

            }
            if ui.texture(self.textures.get(&1).unwrap().clone(), size, size) {

            }
            if ui.texture(self.textures.get(&1).unwrap().clone(), size, size) {

            }
            
        });
    }
}