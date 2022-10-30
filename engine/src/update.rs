use context::{EntityKey, slotmap::SlotMap, Entity, Map, glam::{Vec2, Vec3}, Command};
use parry2d::{na::Isometry2, bounding_volume::BoundingVolume};

use crate::Engine;



fn move_entity(key:EntityKey, e:&mut Entity, v:Vec3, entities:&mut SlotMap<EntityKey, Entity>, map:&Map, commands:&mut Vec<Command>) {
    const DIMS:[Vec2;2] = [Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)];
    let _step_size = 1.0/16.0;
    for dim in DIMS {
        let pos_org = e.pos;
        let v = v.truncate() * dim;
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
                    commands.push(Command::ContactEntity { entity: key.clone(), other: other_key.clone() });
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
                        commands.push(Command::ContactTile { entity: key.clone(), tile: np });
                        break;
                    }
                }
            }

        }
   
        e.pos = pos_new;
        *entities.get_mut(key).unwrap() = *e;
    }
}


impl Engine {
    pub fn update(&mut self) {
        let ctx = &mut self.ctx;
        let mut entities = ctx.entities.clone();

        // move entities that have velocity
        for (key, e) in ctx.entities.iter_mut() {
            let v = e.vel;
            if v.length() > 0.0 {
                let mut left = v.length();
                let d = v.normalize();

                // max step should be configurable at some point
                let max_step = 1.0 / 16.0;
                while left > 0.0 {
                    let mut step = left;
                    if step > max_step {
                        step = max_step;
                    }
                    let v = d * step;
                    left -= step;
                    move_entity(key, e, v, &mut entities, &ctx.map, &mut ctx.commands);
                }
            }
        }
    }
}