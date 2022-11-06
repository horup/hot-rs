use glam::Vec2;
use crate::{Id, Entity, Map, DrawStringParams, DrawTextureParams, DrawRectParams, Command, Camera};

pub trait Engine {
    fn spawn_entity(&mut self, entity:Entity) -> Id;
    fn despawn_entity(&mut self, id:Id);
    fn clear(&mut self);
    fn entity(&self, id:Id) -> Option<&Entity>;
    fn entity_mut(&self, id:Id) -> Option<&mut Entity>;
    fn map(&self) -> &Map;
    fn draw_world(&mut self, camera:&Camera);
    fn screen_size(&self) -> Vec2;
    fn texture_size(&self, texture:u32) -> Vec2;
    fn draw_string(&self, params:DrawStringParams);
    fn draw_texture(&self, params:DrawTextureParams);
    fn draw_rect(&self, params:DrawRectParams);
    fn push_command(&mut self, command:Command);
}