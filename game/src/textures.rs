use num_enum::{TryFromPrimitive, IntoPrimitive};

#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
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