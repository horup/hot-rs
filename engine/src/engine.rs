use std::{fs::Metadata, path::{PathBuf, Path}, collections::HashMap, time::Duration, cell::UnsafeCell, borrow::BorrowMut};

use shared::{Context, CanvasOrg, Id, Entity, slotmap::SlotMap, glam::Vec2, Engine, Game};
use libloading::{Library, Symbol};
use macroquad::{texture::Texture2D, time::get_frame_time, window::{screen_width, screen_height}};
use native_dialog::FileDialog;

#[derive(Default)]
pub struct MacroquadEngine {
    pub game:Option<Box<dyn Game>>,
    entities: SlotMap<Id, UnsafeCell<Entity>>,
    pub game_lib_path: PathBuf,
    pub game_lib: Option<Library>,
    pub game_lib_metadata: Option<Metadata>,
    pub ctx:Context,
    pub textures:HashMap<u32, Texture2D>,
    pub flash_timer:f32,
    pub flash_timer_start:f32
}

impl shared::Engine for MacroquadEngine {
    fn spawn_entity(&mut self, entity:Entity) -> Id {
        let id = self.entities.borrow_mut().insert(UnsafeCell::new(entity));
        return id;
    }

    fn despawn_entity(&mut self, id:Id) {
        self.entities.remove(id);
    }

    fn clear(&mut self) {
        self.entities.clear();
    }

    fn entity(&self, id:Id) -> Option<&Entity> {
        let e = self.entities.get(id);
        if let Some(e) = e {
            unsafe {
                let e = e.get().as_mut().unwrap();
                return Some(e);
            }
        }
        return None;
    }

    fn entity_mut(&self, id:Id) -> Option<&mut Entity> {
        let e = self.entities.get(id);
        if let Some(e) = e {
            unsafe {
                let e = e.get().as_mut().unwrap();
                return Some(e);
            }
        }
        return None;
    }

    fn map(&self) -> &shared::Map {
        &self.ctx.map
    }

    fn draw_world(&mut self) {
        self.draw();
    }

    fn screen_size(&self) -> shared::glam::Vec2 {
        Vec2::new(screen_width(), screen_height())
    }

    fn texture_size(&self, texture:u32) -> shared::glam::Vec2 {
        if let Some(tex) = self.textures.get(&texture) {
            return Vec2::new(tex.width(), tex.height());
        }

        Vec2::new(0.0, 0.0)
    }

    fn draw_string(&self, params:shared::DrawStringParams) {
        todo!()
    }

    fn draw_texture(&self, params:shared::DrawTextureParams) {
        todo!()
    }

    fn draw_rect(&self, params:shared::DrawRectParams) {
        todo!()
    }

    fn push_command(&mut self, command:shared::Command) {
        self.ctx.commands.push(command);
    }
}

impl MacroquadEngine {
    pub fn new(game_path: PathBuf) -> Self {
        Self {
            game_lib_path: game_path,
            ..Default::default()
        }
    }

    pub fn save_map_to_file(&self) {
        let path = FileDialog::new()
        .add_filter("Map file", &["map"])
        .show_save_single_file().unwrap();
        if let Some(path) = path {
            let json = serde_json::to_string(&self.ctx.map).unwrap();
            std::fs::write(path, json).unwrap();
        }
    }

    pub fn load_map_from_file(&mut self) {
        let path = FileDialog::new()
        .add_filter("Map file", &["map"])
        .show_open_single_file().unwrap();
        if let Some(path) = path {
            self.load_map_from_path(path);
        }
    }

    pub fn load_map_from_path(&mut self, path:impl AsRef<Path>) {
        let json = std::fs::read_to_string(path).unwrap();
        self.ctx.map = serde_json::from_str(&json).unwrap();
    }

    pub fn poll_game_lib(&mut self) {
        let metadata = std::fs::metadata(&self.game_lib_path);
        if let Ok(metadata) = metadata {
            let mut load_new = false;
            let mut unload = false;
            if self.game_lib_metadata.is_none()
                || self.game_lib_metadata.clone().unwrap().modified().unwrap()
                    != metadata.modified().unwrap()
            {
                load_new = true;
                if self.game_lib.is_some() {
                    unload = true;
                }
            }

            let mut state:Vec<u8> = Vec::new();
            if unload {
                state = self.game.take().unwrap().serialize();
                self.ctx.entities.clear();

                if let Some(lib) = self.game_lib.take() {
                    lib.close().unwrap();
                }
            }
            
            let load_new = load_new;
            while load_new {
                let mut to = std::env::current_exe().unwrap();
                to.pop();
                to.push("hot.module");
                
                if std::fs::copy(&self.game_lib_path.clone(), to.clone()).is_ok() {
                    unsafe {
                        let lib = libloading::Library::new(to.clone());
                        match lib {
                            Ok(lib) => {
                                self.game_lib_metadata = Some(metadata);
                                self.game_lib = Some(lib);
                                println!("Game lib loaded");
                                let mut s = self.call_game_create().unwrap();

                                if unload {
                                    s.deserialize(&state);
                                }

                                self.game = Some(s);

                                break;
                            },
                            Err(err) => {
                                println!("Could not load game lib with err:{:?}", err);
                                break;
                            },
                        }
                    }
                } else {
                    // retry
                    std::thread::sleep(Duration::from_millis(1000));
                    continue;
                }
            }
        } else {
            println!("Could not load metadata of game lib");
        }
    }

    pub fn call_game_create(&mut self) -> Option<Box<dyn Game>> {
        if let Some(lib) = self.game_lib.take() {
            unsafe {
                if let Ok(f) = lib.get::<fn(state:&mut dyn Engine) -> Box<dyn Game>>(b"create") {
                    let game = f(self);
                    self.game_lib = Some(lib);
                    return Some(game);
                }
            }
            self.game_lib = Some(lib);
        }

        return None;
    }


    pub async fn tick(&mut self) {
        let prev_edit_mode = self.ctx.edit_mode;
        self.ctx.dt = get_frame_time();
        self.input();
        self.process_commands().await;
        let edit_mode_changed = prev_edit_mode != self.ctx.edit_mode;

        if !self.ctx.edit_mode {
            if edit_mode_changed {
//                self.call_game_start();
            }
  //          self.call_game_update();
    //        self.update();
      //      self.call_game_post_update();

            let game = self.game.take();
            if let Some(mut game) = game {
                game.tick(self);
                self.game = Some(game);
            }
        }

        self.process_commands().await;
        self.draw();

        self.ui();
    }
}