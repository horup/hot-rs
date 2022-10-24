use context::{Command, Context};

pub enum Textures {
    Player = 1,
    Piggy = 2,
    Key = 3,
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
    WhiteDoorSide = 17
}
impl Into<u32> for Textures {
    fn into(self) -> u32 {
        self as u32
    }
}

#[no_mangle]
pub fn init(ctx: &mut Context) {
    ctx.edit_mode = true;
    ctx.debug = true;

    ctx.define_texture(Textures::Player, "assets/textures/guy3.png");
    ctx.define_texture(Textures::Piggy, "assets/textures/piggy4.png");
    ctx.define_texture(Textures::Key, "assets/textures/key3.png");
    ctx.define_texture(
        Textures::TiledFloorGray,
        "assets/textures/tiled_floor_gray.png",
    );
    ctx.define_texture(Textures::GrassFloor, "assets/textures/grass_floor.png");
    ctx.define_texture(Textures::BushWall, "assets/textures/bush_wall.png");
    ctx.define_texture(Textures::BrickWall, "assets/textures/brick_wall_red.png");
    ctx.define_texture(Textures::BlackWall, "assets/textures/black_wall.png");
    ctx.define_texture(Textures::WhiteWall, "assets/textures/white_wall.png");
    ctx.define_texture(Textures::WoodFloor, "assets/textures/wood_floor.png");

    ctx.define_texture(Textures::William, "assets/textures/william.png");
    ctx.define_texture(Textures::Viktor, "assets/textures/viktor.png");
    ctx.define_texture(Textures::PokemonCard, "assets/textures/pokemon_card.png");
    ctx.define_texture(Textures::WhiteDoor, "assets/textures/white_door.png");
    ctx.define_texture(Textures::WhiteDoorSide, "assets/textures/white_door_side.png");

    let edit = &mut ctx.edit;
    edit.entities = [
        Textures::William.into(),
        Textures::Viktor.into(),
        Textures::PokemonCard.into(),
        Textures::WhiteDoor.into(),
        Textures::WhiteDoorSide.into()
    ]
    .to_vec()
    .into();

    edit.tiles = [
        Textures::TiledFloorGray.into(),
        Textures::GrassFloor.into(),
        Textures::BushWall.into(),
        Textures::BrickWall.into(),
        Textures::BlackWall.into(),
        Textures::WhiteWall.into(),
        Textures::WoodFloor.into()
    ]
    .into();
}

#[no_mangle]
pub fn update(engine: &mut Context) {
    if engine.input.action {
        engine.commands.push(Command::FlashScreen {});
    }
}
