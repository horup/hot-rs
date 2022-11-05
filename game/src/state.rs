use context::{Id, slotmap::SecondaryMap};
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
    pub player:Option<Id>,
    pub walkers:SecondaryMap<Id, Walker>,
    pub doors:SecondaryMap<Id, Door>,
    pub flash_timer_sec:f32,
    pub flash_timer_start:f32,
    pub flash_max:f32,
}

impl State {
    pub fn flash(&mut self, duration_sec:f32, flash_max:f32) {
        self.flash_timer_sec = duration_sec;
        self.flash_timer_start = duration_sec;
        self.flash_max = flash_max;
    }
}


#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Walker {
    pub walker:f32,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Door {
    pub open:bool,
    pub close_timer_sec:f32,
}

impl Door {
    pub fn open_door(&mut self) {
        self.open = true;
        self.close_timer_sec = 5.0; // close after 5 sec
    }
}


pub static mut STATE:Option<State> = None; 

pub fn state_mut() -> &'static mut State {
    unsafe {
        return STATE.as_mut().unwrap();
    }
}