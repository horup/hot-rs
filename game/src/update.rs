use context::{Context, Command};

use crate::STATE;

#[no_mangle]
pub fn update(ctx: &mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    let dt = ctx.dt; 

    if let Some(index) = state.player {
        if let Some(e) = ctx.entities.get_mut(index) {
            let speed = 6.0;
            let v = ctx.input.dir * speed * dt;
            e.pos += v.extend(0.0);
            ctx.game_camera.zoom = 8.0;
            ctx.game_camera.pos = e.pos.truncate(); 
        }
    } 

    if ctx.input.action {
        ctx.commands.push(Command::FlashScreen {});
    }
}
