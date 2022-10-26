use context::{Command, Context};

pub enum Textures {
    Piggy = 2,
    TiledFloorGray = 6,
    GrassFloor = 7,
    BushWall = 8,
    BrickWall = 9,
    BlackWall = 10,
    WhiteWall = 11,
    WoodFloor = 12,
    William = 13,
    Viktor = 14,
    PokemonCard = 15,
    WhiteDoor = 16,
    WhiteDoorSide = 17,
    Plant = 18,
    HappyPoster = 19,
    GoldDoor = 20,
    GoldKey = 21,
    BlueDoor = 22,
    BlueKey = 23,
    WaypointMarker = 24,
    ExitMarker = 25
}
impl Into<u32> for Textures {
    fn into(self) -> u32 {
        self as u32
    }
}

#[no_mangle]
pub fn start(ctx:&mut Context) {
    println!("Starting game");
}

#[no_mangle]
pub fn init(ctx: &mut Context) {
    ctx.edit_mode = true;
    ctx.debug = true;
    ctx.state.camera.zoom = 16.0;
   
    let mut tiles:Vec<u32> = Vec::new();
    macro_rules! def_tile {
        ($handle:expr, $path:expr) => {
            ctx.define_texture($handle, $path);
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
            ctx.define_texture($handle, $path);
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

    let edit = &mut ctx.edit;
    edit.entities = entities.into();
    edit.tiles = tiles.into();


    ctx.commands.push(Command::LoadMap { map_path:"assets/maps/test.map".into()});
}

#[no_mangle]
pub fn update(engine: &mut Context) {
    if engine.input.action {
        engine.commands.push(Command::FlashScreen {});
    }
}
