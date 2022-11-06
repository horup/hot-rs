mod textures;
pub use textures::*;

use shared::*;

#[derive(Default)]
pub struct MyGame {

}

impl Game for MyGame {
    fn tick(&mut self, engine:&mut dyn Engine) {
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    fn deserialize(&mut self, vec:&[u8]) {
    }

    fn init(&mut self, engine:&mut dyn Engine) {
        let mut tiles:Vec<u32> = Vec::new();
        macro_rules! def_tile {
            ($handle:expr, $path:expr) => {
                engine.push_command(Command::DefineTexture { handle: $handle.into(), path: $path.into() });
                tiles.push($handle.into());
            };
        }
        def_tile!(Textures::TiledFloorGray, "assets/textures/tiled_floor_gray.png");
        def_tile!(Textures::GrassFloor, "assets/textures/grass_floor.png");
        def_tile!(Textures::BushWall, "assets/textures/bush_wall.png");
        def_tile!(Textures::BrickWall, "assets/textures/brick_wall_red.png");
        def_tile!(Textures::BlackWall, "assets/textures/black_wall.png");
        def_tile!(Textures::WhiteWall, "assets/textures/white_wall.png");
        def_tile!(Textures::WoodFloor, "assets/textures/wood_floor.png");
    
        let mut entities:Vec<u32> = Vec::new();
        macro_rules! def_entity {
            ($handle:expr, $path:expr) => {
                engine.push_command(Command::DefineTexture { handle: $handle.into(), path: $path.into() });
                entities.push($handle.into());
            };
        }
        def_entity!(Textures::William, "assets/textures/william.png");
        def_entity!(Textures::Viktor, "assets/textures/viktor.png");
        def_entity!(Textures::PokemonCard, "assets/textures/pokemon_card.png");
        def_entity!(Textures::WhiteDoor, "assets/textures/white_door.png");
        def_entity!(Textures::WhiteDoorSide, "assets/textures/white_door_side.png");
        def_entity!(Textures::Plant, "assets/textures/plant.png");
        def_entity!(Textures::HappyPoster, "assets/textures/happy_poster.png");
        def_entity!(Textures::Piggy, "assets/textures/piggy.png");
        def_entity!(Textures::GoldDoor, "assets/textures/gold_door.png");
        def_entity!(Textures::GoldKey, "assets/textures/gold_key.png");
        def_entity!(Textures::BlueDoor, "assets/textures/blue_door.png");
        def_entity!(Textures::BlueKey, "assets/textures/blue_key.png");
        def_entity!(Textures::WaypointMarker, "assets/textures/waypoint_marker.png");
        def_entity!(Textures::ExitMarker, "assets/textures/exit_marker.png");

        engine.push_command(Command::LoadMap { map_path: "assets/maps/test.map".into() });
    }
}


#[no_mangle]
pub fn create(engine:&mut dyn Engine) -> Box<dyn Game> {
    Box::new(MyGame::default())
}

/*mod state;
use context::{Context, slotmap::SlotMap, Id, Entity};
use serde::{Serialize, Deserialize};
pub use state::*;

use crate::STATE;

#[derive(Serialize, Deserialize)]
struct S {
    pub entities:SlotMap<Id, Entity>,
    pub state:State
}

#[no_mangle]
pub fn serialize(ctx:&mut Context) -> Vec<u8> {
    if let Some(state) = unsafe { STATE.clone()} {
        let v = bincode::serialize(&S {
            entities:ctx.entities.clone(),
            state
        }).unwrap();
        return v;
    }
 
    Vec::new()
}

#[no_mangle]
pub fn deserialize(ctx:&mut Context, state:&Vec<u8>) {
    let s:S = bincode::deserialize(state).unwrap(); 
    ctx.entities = s.entities;
    unsafe {STATE = Some(s.state)}
}
 
mod init;
pub use init::*;

mod start;
pub use start::*;

mod update;
pub use update::*;

mod draw;
pub use draw::*;

mod post_update;
pub use post_update::*;*/