use context::{Context, Command, glam::Vec2, Entity, slotmap::SlotMap, Map, EntityKey};

use crate::STATE;


fn move_entity(e:&mut Entity, v:Vec2, entities:&mut SlotMap<EntityKey, Entity>, map:&Map) {

    const dims:[Vec2;2] = [Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)];

    for dim in dims {
        let v = v * dim;
        e.pos += v.extend(0.0);
    }
}

#[no_mangle]
pub fn update(ctx: &mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    let dt = ctx.dt; 
    let mut entities = ctx.entities.clone();

    for (key, e) in ctx.entities.iter_mut() {
      
    }


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

        if v.length() > 0.0 {
            move_entity(e, v, &mut entities, &ctx.map)
        }
        
        if state.player == Some(key) {
            ctx.game_camera.zoom = 8.0;
            ctx.game_camera.pos = e.pos.truncate(); 
        }
    }

    if ctx.input.action {
        ctx.commands.push(Command::FlashScreen {});
    }
}
