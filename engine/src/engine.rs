use std::{
    borrow::BorrowMut,
    cell::UnsafeCell,
    collections::HashMap,
    fs::Metadata,
    path::{Path, PathBuf},
    time::Duration,
};

use libloading::{Library, Symbol};
use macroquad::{
    texture::Texture2D,
    time::get_frame_time,
    window::{screen_height, screen_width},
};
use native_dialog::FileDialog;
use shared::{
    glam::Vec2, slotmap::SlotMap, Camera, Command, Context, Edit, Entity, Game, Id, Map,
    PlayerInput,
};

#[derive(Default)]
pub struct Engine {
    pub(crate) over_ui: bool,
    pub(crate) edit: Edit,
    pub(crate) input: PlayerInput,
    pub(crate) commands: Vec<Command>,
    pub(crate) game_camera: Camera,
    pub(crate) edit_camera: Camera,
    pub(crate) edit_mode: bool,
    pub(crate) map: Map,
    pub(crate) game: Option<Box<dyn Game>>,
    pub(crate) entities: SlotMap<Id, UnsafeCell<Entity>>,
    pub(crate) game_lib_path: PathBuf,
    pub(crate) game_lib: Option<Library>,
    pub(crate) game_lib_metadata: Option<Metadata>,
    pub(crate) textures: HashMap<u32, Texture2D>,
}

impl Engine {
    pub fn new(game_path: PathBuf) -> Self {
        Self {
            game_lib_path: game_path,
            ..Default::default()
        }
    }

    pub fn save_map_to_file(&self) {
        let path = FileDialog::new()
            .add_filter("Map file", &["map"])
            .show_save_single_file()
            .unwrap();
        if let Some(path) = path {
            let json = serde_json::to_string(&self.map).unwrap();
            std::fs::write(path, json).unwrap();
        }
    }

    pub fn load_map_from_file(&mut self) {
        let path = FileDialog::new()
            .add_filter("Map file", &["map"])
            .show_open_single_file()
            .unwrap();
        if let Some(path) = path {
            self.load_map_from_path(path);
        }
    }

    pub fn load_map_from_path(&mut self, path: impl AsRef<Path>) {
        let json = std::fs::read_to_string(path).unwrap();
        self.map = serde_json::from_str(&json).unwrap();
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

            let mut state: Vec<u8> = Vec::new();
            if unload {
                state = self.game.take().unwrap().serialize();
                self.entities.clear();

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
                            }
                            Err(err) => {
                                println!("Could not load game lib with err:{:?}", err);
                                break;
                            }
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
                if let Ok(f) = lib.get::<fn(state: &mut dyn Context) -> Box<dyn Game>>(b"create") {
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
        let prev_edit_mode = self.edit_mode;
        self.input();
        self.process_commands().await;
        let edit_mode_changed = prev_edit_mode != self.edit_mode;

        if !self.edit_mode {
            let game = self.game.take();
            if let Some(mut game) = game {
                if edit_mode_changed {
                    //                self.call_game_start();
                }

                game.tick(self);
                self.game = Some(game);
            }
        } else {
            self.draw_edit_mode();
        }

        self.process_commands().await;

        self.ui();
    }
}
