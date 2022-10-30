use context::{EntityKey, slotmap::SecondaryMap};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde::{Serialize, Deserialize};

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

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct State {
    pub player:Option<EntityKey>,
    pub walkers:SecondaryMap<EntityKey, Walker>,

}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Walker {
    pub walker:f32,
}

pub static mut STATE:Option<State> = None; 