use std::cell::UnsafeCell;

use macroquad::{prelude::Vec2, window::{screen_width, screen_height}};
use shared::{Entity, Id, Camera, Context};

use crate::Engine;


impl Context for Engine {
    fn spawn_entity(&mut self, entity:Entity) -> Id {
        let id = self.entities.insert(UnsafeCell::new(entity));
        return id;
    }

    fn despawn_entity(&mut self, id:Id) {
        self.entities.remove(id);
    }

    fn clear(&mut self) {
        self.entities.clear();
    }

    fn entity(&self, id:Id) -> Option<&Entity> {
        let e = self.entities.get(id);
        if let Some(e) = e {
            unsafe {
                let e = e.get().as_mut().unwrap();
                return Some(e);
            }
        }
        return None;
    }

    fn entity_mut(&self, id:Id) -> Option<&mut Entity> {
        let e = self.entities.get(id);
        if let Some(e) = e {
            unsafe {
                let e = e.get().as_mut().unwrap();
                return Some(e);
            }
        }
        return None;
    }

    fn map(&self) -> &shared::Map {
        &self.map
    }

    fn draw_world(&mut self, camera:&Camera) {
        self.game_camera = camera.clone();
        self.draw_game_mode();
    }

    fn screen_size(&self) -> shared::glam::Vec2 {
        Vec2::new(screen_width(), screen_height())
    }

    fn texture_size(&self, texture:u32) -> shared::glam::Vec2 {
        if let Some(tex) = self.textures.get(&texture) {
            return Vec2::new(tex.width(), tex.height());
        }

        Vec2::new(0.0, 0.0)
    }

    fn draw_string(&self, params:shared::DrawStringParams) {
        todo!()
    }

    fn draw_texture(&self, params:shared::DrawTextureParams) {
        todo!()
    }

    fn draw_rect(&self, params:shared::DrawRectParams) {
        todo!()
    }

    fn push_command(&mut self, command:shared::Command) {
        self.commands.push(command);
    }
}