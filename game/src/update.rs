use context::{Context, Command, glam::{Vec2}, Entity, slotmap::SlotMap, Map, EntityKey};
use parry2d::{na::Isometry2, bounding_volume::BoundingVolume};

use crate::STATE;


fn move_entity(key:EntityKey, e:&mut Entity, v:Vec2, entities:&mut SlotMap<EntityKey, Entity>, map:&Map) {
    const DIMS:[Vec2;2] = [Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)];
    let _step_size = 1.0/16.0;
    for dim in DIMS {
        let pos_org = e.pos;
        let v = v * dim;
        if v.length() == 0.0 {
            continue;
        }

        let mut pos_new = pos_org + v.extend(0.0);

        // collision handling between entities
        for (other_key, other_e) in entities.iter() {
            if other_key != key {
                let d = e.pos - other_e.pos;
                let r2 = e.radius + other_e.radius;
                if d.length() < r2 {
                    let r = r2 - d.length();
                    let v = d.normalize() * r;
                    pos_new += v;
                }
            }
        }

        // collision between grid
        let v = pos_new - pos_org; 
        let v = v.truncate() * dim;
        let d = v.normalize();
        let rev_dim = Vec2::new(dim.y, dim.x);
        for i in [-1, 0, 1] {
            let i = i as f32;
            let cp = Vec2::new(i, i) * rev_dim + d + pos_org.truncate();
            let _vvv = cp - pos_org.truncate();
            let np = cp.as_ivec2();
            if let Some(cell) = map.grid.get(np.x, np.y) {
                if cell.blocks {
                    let s1 = parry2d::shape::Cuboid::new([e.radius, e.radius].into());
                    let s1_pos = Isometry2::translation(pos_new.x, pos_new.y);
                    let aabb1 = s1.aabb(&s1_pos);
                    let s2 = parry2d::shape::Cuboid::new([0.5, 0.5].into());
                    let s2_pos = Isometry2::translation(np.x as f32 + 0.5, np.y as f32 + 0.5);
                    let aabb2 = s2.aabb(&s2_pos);
 
                    if aabb1.intersects(&aabb2) {
                        pos_new = pos_org;
                        break;
                    }
                }
            }

        }
   
        e.pos = pos_new;
        *entities.get_mut(key).unwrap() = *e;
    }
}

#[no_mangle]
pub fn update(ctx: &mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    let dt = ctx.dt; 
    let mut entities = ctx.entities.clone();

    for (_key, _e) in ctx.entities.iter_mut() {
      
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
            move_entity(key, e, v, &mut entities, &ctx.map)
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
