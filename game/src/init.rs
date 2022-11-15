use shared::*;
use crate::{Images, sounds};

pub fn init(engine: &mut dyn Context) {
    let mut tiles:Vec<u32> = Vec::new();
    macro_rules! def_tile {
        ($handle:expr, $path:expr) => {
            engine.push_command(Command::DefineImg { handle: $handle.into(), path: $path.into() });
            tiles.push($handle.into());
        };
    }
    def_tile!(Images::TiledFloorGray, "assets/textures/tiled_floor_gray.png");
    def_tile!(Images::GrassFloor, "assets/textures/grass_floor.png");
    def_tile!(Images::BushWall, "assets/textures/bush_wall.png");
    def_tile!(Images::BrickWall, "assets/textures/brick_wall_red.png");
    def_tile!(Images::BlackWall, "assets/textures/black_wall.png");
    def_tile!(Images::WhiteWall, "assets/textures/white_wall.png");
    def_tile!(Images::WoodFloor, "assets/textures/wood_floor.png");
    let mut entities:Vec<u32> = Vec::new();
    macro_rules! def_entity {
        ($handle:expr, $path:expr) => {
            engine.push_command(Command::DefineImg { handle: $handle.into(), path: $path.into() });
            entities.push($handle.into());
        };
    }
    def_entity!(Images::William, "assets/textures/william.png");
    def_entity!(Images::Viktor, "assets/textures/viktor.png");
    def_entity!(Images::PokemonCard, "assets/textures/pokemon_card.png");
    def_entity!(Images::WhiteDoor, "assets/textures/white_door.png");
    def_entity!(Images::WhiteDoorSide, "assets/textures/white_door_side.png");
    def_entity!(Images::Plant, "assets/textures/plant.png");
    def_entity!(Images::HappyPoster, "assets/textures/happy_poster.png");
    def_entity!(Images::Piggy, "assets/textures/piggy.png");
    def_entity!(Images::GoldDoor, "assets/textures/gold_door.png");
    def_entity!(Images::GoldKey, "assets/textures/gold_key.png");
    def_entity!(Images::BlueDoor, "assets/textures/blue_door.png");
    def_entity!(Images::BlueKey, "assets/textures/blue_key.png");
    def_entity!(Images::WaypointMarker, "assets/textures/waypoint_marker.png");
    def_entity!(Images::ExitMarker, "assets/textures/exit_marker.png");

    macro_rules! def_sound {
        ($handle:expr, $path:expr) => {
            engine.push_command(Command::DefineSound { handle: $handle, path: $path.into() });
        };
    }

    engine.push_command(Command::DefineSound { handle: sounds::PICKUP, path: "assets/sfx/pickup.ogg".into() });
    engine.push_command(Command::DefineSound { handle: sounds::DOOR_OPEN, path: "assets/sfx/door_open.ogg".into() });
    engine.push_command(Command::DefineSound { handle: sounds::DOOR_CLOSE, path: "assets/sfx/door_close.ogg".into() });
    def_sound!(sounds::PICKUP_KEY, "assets/sfx/pickup_key.ogg");
}
