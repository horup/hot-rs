use glam::Vec2;
use crate::Context;

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

pub trait Canvas {
    fn ctx_mut(&mut self) -> &mut Context;
    fn screen_size(&mut self) -> Vec2;
    fn texture_size(&mut self, texture:u32) -> Vec2;
    fn draw_string(&mut self, params:DrawStringParams);
    fn draw_texture(&mut self, params:DrawTextureParams);
}