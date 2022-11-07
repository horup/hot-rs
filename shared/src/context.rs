use glam::Vec2;
use crate::{Id, Entity, Map, Command, Camera, Event};

#[derive(Default, Clone)]
pub struct DrawStringParams {
    pub str:String,
    pub x:f32,
    pub y:f32
}

#[derive(Default, Clone, Copy)]
pub struct DrawTextureParams {
    pub texture:u32,
    pub x:f32,
    pub y:f32,
    pub w:f32,
    pub h:f32
}

#[derive(Default, Clone, Copy)]
pub struct DrawRectParams {
    pub x:f32,
    pub y:f32,
    pub w:f32,
    pub h:f32,
    pub color:Color
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r:f32, g:f32, b:f32, a:f32) -> Self {
        Color {r,g,b,a}
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r:1.0,
            g:1.0,
            b:1.0,
            a:1.0
        }
    }
}

pub trait Context {
    fn key_pressed(&self, key_code:i32) -> bool;
    fn key_down(&self, key_code:i32) -> bool;
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
    fn events(&mut self) -> Vec<Event>;
}