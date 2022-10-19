#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub x:f32,
    pub y:f32,
    pub texture:u32
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub pos:Vec2,
    pub zoom:f32
}
impl Default for Camera {
    fn default() -> Self {
        Self { pos: Default::default(), zoom: 1.0 / 8.0 }
    }
}

#[derive(Default, Debug, Clone)]
pub struct State {
    pub camera:Camera,
    pub iterations:u64,
    pub entities:Vec<Entity>,
}

#[derive(Default)]
pub struct PlayerInput {
    pub dir:Vec2,
    pub action:bool
}

#[derive(Default)]
pub struct Context {
    pub state:State,
    pub commands:Vec<Command>,
    pub player_input:PlayerInput,
    pub debug:bool
}

impl Context {
    pub fn define_texture(&mut self, handle:u32, src:&str) {
        self.commands.push(Command::DefineTexture { handle: handle, path: src.into() })
    }
}


mod command;
pub use command::*;
use glam::Vec2;

pub use glam;