use context::{Context, Command, glam::{Vec2}, IgnoreColissions};
use crate::STATE;


#[no_mangle]
pub fn update(ctx: &mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    let dt = ctx.dt; 

    for (key, e) in ctx.entities.iter_mut() {
        let speed = 3.0;
        let mut v = Vec2::default();
        if state.player == Some(key) {
            v = ctx.input.dir * speed * dt;
        }

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
            let walking = v.length() > 0.0;
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

        e.vel = v.extend(0.0);
        
        if state.player == Some(key) {
            ctx.game_camera.zoom = 12.0;
            ctx.game_camera.pos = e.pos.truncate(); 
        }
    }

    for (key, e) in ctx.entities.iter_mut() {
        if let Some(door) = state.doors.get_mut(key) {
            if door.open == true {
                e.ignore_collisions = IgnoreColissions::WithEntities;
                e.hidden = true;
            } else {
                e.hidden = false;
            }

            door.close_timer_sec -= dt;
            if door.close_timer_sec <= 0.0 {
                door.close_timer_sec = 0.0;
                e.ignore_collisions = IgnoreColissions::None;
                door.open = false;
            }
        }
    }

    if ctx.input.action {
        ctx.commands.push(Command::FlashScreen {});
    }
}


#[no_mangle]
pub fn post_update(context:&mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    for c in context.commands.iter() {
        match c {
            Command::ContactEntity { entity: _, other } => {
                if let Some(door) = state.doors.get_mut(*other) {
                    door.open_door();
                }
            },
            Command::ContactTile { entity: _, tile: _ } => {
            },
            _=>{}
        }
    }
}