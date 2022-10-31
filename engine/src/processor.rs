use context::{Command, Context};
use libloading::{Symbol};
use macroquad::texture::{Texture2D, load_texture, FilterMode};
use crate::Engine;


impl Engine {
    pub async fn process_commands(&mut self) {
        let commands:Vec<Command> = self.ctx.commands.drain(..).collect();
        for command in commands.iter() {
            match command {
                Command::Restart => {
                    if let Some(lib) = &self.game_lib {
                        self.ctx = Context::default();
                        unsafe {
                            let init_func:Symbol<fn(state:&mut Context)> = lib.get(b"init").unwrap();
                            init_func(&mut self.ctx);
                        }

                        if !self.ctx.edit_mode {
                            self.call_game_start();
                        }
                    }
                }
                Command::DefineTexture { handle, path } => {
                    let texture: Texture2D = load_texture(path).await.unwrap();
                    texture.set_filter(FilterMode::Nearest);
                    self.textures.insert(*handle, texture);
                },
                Command::LoadMap { map_path } => {
                    self.load_map_from_path(map_path);
                    if !self.ctx.edit_mode {
                        self.call_game_start();
                    }
                }
                Command::ContactEntity { entity: _, other: _ } => {},
                Command::ContactTile { entity: _, tile: _ } => {},
            }
        }
    }
}