use context::{Command, Context, Entity, glam::{Vec3}, Grid};


#[no_mangle]
pub fn start(ctx:&mut Context) {
    ctx.game = Box::new(State::default());
    let game = ctx.game.downcast_mut::<State>().unwrap();

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
                game.player = Some(player_entity);
            }
        }
    })
}


#[no_mangle]
pub fn update(ctx: &mut Context) {
    let game = ctx.game.downcast_mut::<State>().unwrap();
    let dt = ctx.dt;


    if let Some(index) = game.player {
        if let Some(e) = ctx.entities.get_mut(index) {
            let speed = 2.0;
            let v = ctx.input.dir * speed * dt;
            e.pos += v.extend(0.0);
            ctx.game_camera.zoom = 8.0;
            ctx.game_camera.pos = e.pos.truncate();
            println!("{:?}", e.pos); 
        }
    }

    if ctx.input.action {
        ctx.commands.push(Command::FlashScreen {});
    }
}


mod init;
pub use init::*;