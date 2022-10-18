/*use std::{path::Path, fs::Metadata};

use context::*;
use libloading::*;
use macroquad::prelude::*;

mod draw;
pub use draw::*;
mod process;
pub use process::*;
mod input;
pub use input::*;
mod engine;
pub use engine::*;

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
}


#[macroquad::main("HOT RELOAD TEST")]
async fn main() {
    let current_exe_path = std::env::current_exe().unwrap();
    let mut lib_path = current_exe_path.parent().unwrap().to_path_buf();
    lib_path.push("game.dll");
    let mut metadata_option:Option<Metadata> = None;
    
    let mut engine = context::Context::default();
    engine.commands.push(context::Command::Restart);
    let mut lib_option:Option<Library> = None;
    loop {
        match lib_option.as_mut() {
            Some(lib) => {
                unsafe {
                    process(&mut engine, lib);
                    input::input(&mut engine);
                    let update_func:Symbol<fn(state:&mut Context)> = lib.get(b"update").unwrap();
                    update_func(&mut engine);
                    draw(&mut engine);
                    process(&mut engine, lib);
                }

                if let Ok(metadata) = std::fs::metadata(&lib_path) {
                    if metadata.modified().unwrap() != metadata_option.clone().unwrap().modified().unwrap() {
                        lib_option = None;
                    }
                }
            },
            None => {
                lib_option = load_lib(&lib_path);
                if lib_option.is_some() {
                    metadata_option = Some(std::fs::metadata(&lib_path).unwrap());
                }
            },
        }
        next_frame().await
    }
}*/