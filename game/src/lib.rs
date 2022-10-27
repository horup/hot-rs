mod state;
pub use state::*;

use context::{Command, Context, Entity, glam::{Vec3}, Grid};

use crate::STATE;

#[no_mangle]
pub fn serialize() -> Vec<u8> {
    if let Some(state) = unsafe { STATE.as_ref()} {
        let v = bincode::serialize(state).unwrap();
        return v;
    }

    Vec::new()
}

#[no_mangle]
pub fn deserialize(state:&Vec<u8>) {
    unsafe {STATE = Some(bincode::deserialize::<State>(state).unwrap())}
}
 
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
                    texture:Textures::William.into()
                });

                state.player = Some(player_entity);
            }
        }
    })
}
 
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


mod init;
pub use init::*;