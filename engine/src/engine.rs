use std::{fs::Metadata, path::{PathBuf, Path}, collections::HashMap, time::Duration};

use context::Context;
use libloading::{Library, Symbol};
use macroquad::{texture::Texture2D, time::get_frame_time};
use native_dialog::FileDialog;

#[derive(Default)]
pub struct Engine {
    pub game_lib_path: PathBuf,
    pub game_lib: Option<Library>,
    pub game_lib_metadata: Option<Metadata>,
    pub ctx:Context,
    pub textures:HashMap<u32, Texture2D>,
    pub flash_timer:f32,
    pub flash_timer_start:f32
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
                state = self.call_game_serialize();
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
                                if unload {
                                    self.call_game_deserialize(&state);
                                }
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


    pub fn call_game_start(&mut self) {
        if let Some(lib) = self.game_lib.as_mut() {
            unsafe {
                let update_func:Symbol<fn(state:&mut Context)> = lib.get(b"start").unwrap();
                update_func(&mut self.ctx);
            }
        }
    }

    pub fn call_game_serialize(&mut self) -> Vec<u8> {
        if let Some(lib) = self.game_lib.as_mut() {
            unsafe {
                let f:Symbol<fn(ctx:&mut Context)->Vec<u8>> = lib.get(b"serialize").unwrap();
                return f(&mut self.ctx);
            }
        }

        Vec::new()
    }

    pub fn call_game_deserialize(&mut self, state:&Vec<u8>) {
        if let Some(lib) = self.game_lib.as_mut() {
            unsafe {
                let f:Symbol<fn(ctx:&mut Context, state:&Vec<u8>)> = lib.get(b"deserialize").unwrap();
                f(&mut self.ctx, state);
            }
        }
    }

    pub fn call_game_update(&mut self) {
        if let Some(lib) = self.game_lib.as_mut() {
            unsafe {
                if let Ok(f) = lib.get::<fn(state:&mut Context)>(b"update") {
                    f(&mut self.ctx);
                }
            }
        }
    }

    pub fn call_game_post_update(&mut self) {
        if let Some(lib) = self.game_lib.as_mut() {
            unsafe {
                if let Ok(f) = lib.get::<fn(state:&mut Context)>(b"post_update") {
                    f(&mut self.ctx);
                }
            }
        }
    }

    pub async fn tick(&mut self) {
        let prev_edit_mode = self.ctx.edit_mode;
        self.ctx.dt = get_frame_time();
        self.input();
        self.process_commands().await;
        let edit_mode_changed = prev_edit_mode != self.ctx.edit_mode;

        if !self.ctx.edit_mode {
            if edit_mode_changed {
                self.call_game_start();
            }
            self.call_game_update();
            self.update();
            self.call_game_post_update();
        }

        self.process_commands().await;
        self.draw();

        self.ui();
    }
}

/*
fn load_lib(path:&Path) -> Option<Library> {
    unsafe {
        let mut to = path.parent().unwrap().to_path_buf();
        to.push("hot.dll");
        if std::fs::copy(path, to.clone()).is_ok() {
            let lib = Some(libloading::Library::new(to).unwrap());
            return lib;
        }
        return None;
    }
}*/
