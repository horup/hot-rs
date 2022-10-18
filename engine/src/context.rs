use std::collections::HashMap;
use context::{State, Command};

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
        self.commands.push(Command::DefineTexture { handle: handle, src: src.into() })
    }
}