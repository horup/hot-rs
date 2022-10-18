#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub x:f32,
    pub y:f32,
    pub texture:u32
}

#[derive(Default, Debug, Clone)]
pub struct State {
    pub iterations:u64,
    pub entities:Vec<Entity>
}

#[derive(Default)]
pub struct PlayerInput {
    pub x:f32,
    pub y:f32,
    pub action:bool
}

#[derive(Default)]
pub struct Context {
    pub state:State,
    pub commands:Vec<Command>,
    pub player_input:PlayerInput,
}

impl Context {
    pub fn define_texture(&mut self, handle:u32, src:&str) {
        self.commands.push(Command::DefineTexture { handle: handle, path: src.into() })
    }
}


mod command;
pub use command::*;