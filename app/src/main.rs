use engine::*;
use macroquad::{prelude::*};

#[macroquad::main("Hot!")]
async fn main() {
    println!("{:?}", std::env::current_dir());
    let current_exe_path = std::env::current_exe().unwrap();
    let mut lib_path = current_exe_path.parent().unwrap().to_path_buf();
    lib_path.push("game.dll");
    let mut engine = Engine::new(lib_path);
    engine.ctx.commands.push(context::Command::Restart);
    loop {
        engine.poll_game_lib();
        engine.tick().await;
        next_frame().await
    }
}
