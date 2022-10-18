use std::{fs::Metadata, path::PathBuf};

use context::Context;
use libloading::{Library, Symbol};

pub struct Engine {
    pub game_lib_path: PathBuf,
    pub game_lib: Option<Library>,
    pub game_lib_metadata: Option<Metadata>,
    pub context:Context
}

impl Engine {
    pub fn new(game_path: PathBuf) -> Self {
        Self {
            game_lib_path: game_path,
            game_lib: None,
            game_lib_metadata: None,
            context:Context::default()
        }
    }

    pub fn poll_game_lib(&mut self) {
        let metadata = std::fs::metadata(&self.game_lib_path);
        if let Ok(metadata) = metadata {
            if self.game_lib_metadata.is_none()
                || self.game_lib_metadata.clone().unwrap().modified().unwrap()
                    != metadata.modified().unwrap()
            {
                let mut to = std::env::current_exe().unwrap();
                to.pop();
                to.push("hot.module");

                if std::fs::copy(&self.game_lib_path.clone(), to.clone()).is_ok() {
                    unsafe {
                        let lib = libloading::Library::new(to.clone());
                        if let Ok(lib) = lib {
                            self.game_lib_metadata = Some(metadata);
                            self.game_lib = Some(lib);
                            println!("Game lib loaded");
                        } else {
                            println!("Could not load game lib");
                        }
                    }
                } else {
                    println!("Could not copy game lib to hot path");
                }
            }
        } else {
            println!("Could not load metadata of game lib");
            return;
        }
    }

    pub fn update(&mut self) {
        self.input();
        self.process_commands();

        if let Some(lib) = self.game_lib.as_mut() {
            unsafe {
                let update_func:Symbol<fn(state:&mut Context)> = lib.get(b"update").unwrap();
                update_func(&mut self.context);
            }
        }

        self.process_commands();
        self.draw();

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
