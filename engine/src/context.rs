use std::{cell::UnsafeCell, mem::transmute};

use macroquad::{prelude::{Vec2, KeyCode, is_key_pressed, is_key_down, get_last_key_pressed}, window::{screen_width, screen_height}, time::get_frame_time};
use shared::{Entity, Id, Camera, Context, Event};

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

    fn draw_string(&self, _params:shared::DrawStringParams) {
        todo!()
    }

    fn draw_texture(&self, _params:shared::DrawTextureParams) {
        todo!()
    }

    fn draw_rect(&self, _params:shared::DrawRectParams) {
        todo!()
    }

    fn push_command(&mut self, command:shared::Command) {
        self.commands.push(command);
    }

    fn events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        std::mem::swap(&mut self.events, &mut events);
        return events;
    }

    fn is_key_pressed(&self, key_code:u8) -> bool {
        let key_code:KeyCode = unsafe {transmute(key_code)};
        
        return is_key_pressed(key_code);
    }

    fn is_key_down(&self, key_code:u8) -> bool {
        let key_code:KeyCode = unsafe {transmute(key_code)};
        return is_key_down(key_code);
    }

    fn last_key_pressed(&self) -> Option<u8> {
        if let Some(key_code) = get_last_key_pressed() {
            let k:u8 = unsafe { transmute(key_code) };
            return Some(k);
        }
        None
    }

    fn entities(&self) -> Vec<Id> {
        let v:Vec<Id> = self.entities.keys().collect();
        v
    }

    fn dt(&self) -> f32 {
        get_frame_time()
    }
    
}