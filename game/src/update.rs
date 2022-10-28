use context::{Context, Command, glam::Vec2, Entity, slotmap::SlotMap, Map, EntityKey, rapier2d::prelude::*};

use crate::STATE;


fn move_entity(e:&mut Entity, v:Vec2, entities:&mut SlotMap<EntityKey, Entity>, map:&Map) {
    e.pos += v.extend(0.0);
}

#[no_mangle]
pub fn update(ctx: &mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    let dt = ctx.dt; 
    let mut entities = ctx.entities.clone();
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let physics_hooks = ();
    let event_handler = ();

    for (key, e) in ctx.entities.iter_mut() {
        let body = RigidBodyBuilder::dynamic()
        .translation([e.pos.x, e.pos.y].into())
       // .user_data(key.into())
        .build();
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
           // move_entity(e, v, &mut entities, &ctx.map)
        }
        
        if state.player == Some(key) {
            ctx.game_camera.zoom = 8.0;
            ctx.game_camera.pos = e.pos.truncate(); 
        }
    }

    
    ctx.physics.physics_pipeline.step(&[0.0, 0.0].into(), 
    &IntegrationParameters {
        dt,
        ..Default::default()
    }, 
    &mut ctx.physics.island_manager, 
    &mut ctx.physics.broad_phase, 
    &mut ctx.physics.narrow_phase, 
    &mut rigid_body_set, 
    &mut collider_set, 
    &mut ctx.physics.impulse_joint_set, 
    &mut ctx.physics.multibody_joint_set, 
    &mut ctx.physics.ccd_solver, 
    &physics_hooks, 
    &event_handler);


    if ctx.input.action {
        ctx.commands.push(Command::FlashScreen {});
    }
}
