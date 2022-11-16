use std::cell::UnsafeCell;

use crate::{Camera, Command, Entities, Entity, Event, Id, Map, Tiles, world, World};
use glam::{Vec2, Vec3, IVec2};
use serde::{Serialize, Deserialize};
use slotmap::SlotMap;

#[derive(Clone)]
pub struct DrawStringParams {
    pub str: String,
    pub x: f32,
    pub y: f32,
    pub font_height:f32,
    pub color:Color
}

impl Default for DrawStringParams {
    fn default() -> Self {
        Self { str: Default::default(), 
            x: Default::default(), 
            y: Default::default(), 
            font_height: 16.0,
            color: Color::default(), }
    }
}

#[derive(Default, Clone, Copy)]
pub struct DrawImgParams {
    pub img: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Default, Clone, Copy)]
pub struct DrawRectParams {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

pub struct EntityIter<'a> {
    pub entities: &'a SlotMap<Id, UnsafeCell<Entity>>,
    pub iter: slotmap::basic::Iter<'a, Id, UnsafeCell<Entity>>,
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = (Id, &'a mut Entity);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Default)]
pub struct Collision {
    pub other_entity:Option<Id>,
    pub tile:Option<IVec2>
}

pub trait Context {
    fn clip_move(&self, id:Id, target:Vec3, world:&World) -> Collision;
    fn is_key_pressed(&self, key_code: u8) -> bool;
    fn is_key_down(&self, key_code: u8) -> bool;
    fn last_key_pressed(&self) -> Option<u8>;
    fn dt(&self) -> f32;
    fn draw(&mut self, camera: &Camera, world:&World);
    fn screen_size(&self) -> Vec2;
    fn texture_size(&self, texture: u32) -> Vec2;
    fn draw_string(&self, params: DrawStringParams);
    fn draw_img(&self, params: DrawImgParams);
    fn draw_rect(&self, params: DrawRectParams);
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, bytes:&[u8]);
    fn push_command(&self, command: Command);
    fn events(&mut self) -> Vec<Event>;
    fn play_sound(&self, sound:u32, volume:f32);
}
