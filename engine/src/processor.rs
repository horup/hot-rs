use shared::{Command, Event};

use macroquad::{texture::{Texture2D, load_texture, FilterMode}, audio::load_sound};
use crate::Engine;


impl Engine {
    pub async fn process_commands(&mut self) {
        let commands:Vec<Command> = self.commands.borrow_mut().drain(..).collect();
        for command in commands.iter() {
            match command {
                Command::DefineImg { handle, path } => {
                    let texture: Texture2D = load_texture(path).await.unwrap();
                    texture.set_filter(FilterMode::Nearest);
                    self.textures.insert(*handle, texture);
                },
                Command::LoadMap { map_path } => {
                    self.load_map_from_path(map_path);
                    self.events.push(Event::MapReady { map:self.map.clone() });
                }
                Command::DefineSound { handle, path } => {
                    let sound = load_sound(path).await.unwrap();
                    self.sounds.insert(*handle, sound);
                },
            }
        }
    }
}