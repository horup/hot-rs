use context::{Context, Command, glam::Vec2};

use crate::STATE;

#[no_mangle]
pub fn update(ctx: &mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    let dt = ctx.dt; 

    if let Some(key) = state.player {
        if let Some(e) = ctx.entities.get_mut(key) {
            let speed = 3.0;
            let v = ctx.input.dir * speed * dt;

            if v.x > 0.0 {
                e.flip_x = false;
            } else if v.x < 0.0 {
                e.flip_x = true;
            }

            if v.length() > 0.0 {
                let d = v.angle_between(Vec2::new(1.0, 0.0));
                e.dir = d;
            }

            if let Some(walker) = state.walkers.get_mut(key) {
                let walking = if v.length() > 0.0 { true } else { false };
                if walking {
                    walker.walker += v.length() * 2.0;
                    if walker.walker > 1.0 {
                        walker.walker = 0.0;
                    }

                    e.pos.z = if walker.walker > 0.5 { 0.1 } else { 0.0 };
                } else {
                    e.pos.z = 0.0;
                }
            }

            e.pos += v.extend(0.0);
            
            ctx.game_camera.zoom = 8.0;
            ctx.game_camera.pos = e.pos.truncate(); 

            //e.pos.y += 0.01 * dt;
        }
    } 

    if ctx.input.action {
        ctx.commands.push(Command::FlashScreen {});
    }
}
