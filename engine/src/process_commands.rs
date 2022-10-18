use context::{Command, Context};
use libloading::{Symbol};
use crate::Engine;


impl Engine {
    pub fn process_commands(&mut self) {
        let commands:Vec<Command> = self.context.commands.drain(..).collect();
        for command in commands.iter() {
            match command {
                Command::Restart => {
                    if let Some(lib) = &self.game_lib {
                        unsafe {
                            let init_func:Symbol<fn(state:&mut Context)> = lib.get(b"init").unwrap();
                            init_func(&mut self.context);
                        }
                    }
                }
                Command::DefineTexture { handle, src } => {

                },
            }
        }
    }
}