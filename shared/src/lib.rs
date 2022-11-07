mod entity;
pub use entity::*;

mod game;
pub use game::*;

mod command;
pub use command::*;

mod map;
pub use map::*;

mod edit;
pub use edit::*;

mod grid;
pub use grid::*;

mod camera;
pub use camera::*;

pub use glam;
use glam::Vec2;

pub use slotmap;
use slotmap::{new_key_type};

mod context;
pub use context::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub texture: u32,
}

#[derive(Default)]
pub struct PlayerInput {
    pub dir: Vec2,
    pub action: bool,
    pub mouse_pos_screen: Vec2,
    pub mouse_pos_world: Vec2,
    pub mouse_left_down: bool,
    pub mouse_right_down: bool,
    pub mouse_left_pressed: bool,
    pub mouse_right_pressed: bool,
}

new_key_type! { pub struct Id; }
