use context::{Context, Grid, glam::Vec3, Entity};

use crate::{Textures, state::STATE, Walker};


#[no_mangle]
pub fn start(ctx:&mut Context) {
    let state = unsafe { STATE.as_mut().unwrap() };
    ctx.entities.clear();
    ctx.tilemap = Grid::default();
    ctx.map.grid.for_each(|cell, x, y| {
        if let Some(entity) = cell.entity { 

            if entity == Textures::William.into() {
                dbg!("Spawning William");
                let player_entity = ctx.entities.insert(Entity {
                    pos:Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                    texture:Textures::William.into(),
                    radius:0.4,
                    ..Default::default()
                });

                state.player = Some(player_entity);
                state.walkers.insert(player_entity, Walker::default());
            } else {
                ctx.entities.insert(Entity {
                    pos:Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                    texture:entity,
                    ..Default::default()
                });
            }
        }
    })
}