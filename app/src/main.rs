use engine::*;
use macroquad::prelude::*;

#[macroquad::main("Hot!")]
async fn main() {
    let current_exe_path = std::env::current_exe().unwrap();
    let mut lib_path = current_exe_path.parent().unwrap().to_path_buf();
    lib_path.push("game.dll");
    let mut engine = Engine::new(lib_path);
    engine.context.commands.push(context::Command::Restart);
    loop {
        engine.poll_game_lib();
        engine.update();
     /*   match lib_option.as_mut() {
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
        }*/
        next_frame().await
    }


}
