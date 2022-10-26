
#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub texture:u32
}


#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub x:f32,
    pub y:f32,
    pub texture:u32
}


#[derive(Default, Debug, Clone)]
pub struct GameState {
    pub iterations:u64,
    pub entities:Vec<Entity>,
    pub tilemap:Grid<Tile>
}

#[derive(Default)]
pub struct PlayerInput {
    pub dir:Vec2,
    pub action:bool,
    pub mouse_pos_screen:Vec2,
    pub mouse_pos_world:Vec2,
    pub mouse_left_down:bool,
    pub mouse_right_down:bool,
    pub mouse_left_pressed:bool,
    pub mouse_right_pressed:bool
}

#[derive(Default)]
pub struct Context {
    pub edit_camera:Camera,
    pub game_camera:Camera,
    pub over_ui:bool,
    pub edit_mode:bool,
    pub map:Map,
    pub game_state:GameState,
    pub commands:Vec<Command>,
    pub input:PlayerInput,
    pub debug:bool,
    pub edit:Edit,
    pub dt:f32
}

impl Context {
    pub fn define_texture(&mut self, handle: impl Into<u32>, src:&str) {
        self.commands.push(Command::DefineTexture { handle: handle.into(), path: src.into() })
    }
}


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

use glam::Vec2;
pub use glam;