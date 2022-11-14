use std::collections::HashMap;

use shared::{Id, Camera, Components};
use serde::{Serialize, Deserialize};

use crate::Textures;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Counter {
    pub current:f32,
    pub total:f32
}
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct State {
    pub camera:Camera,
    pub player:Option<Id>,
    pub walkers:Components<Walker>,
    pub doors:Components<Door>,
    pub items:Components<Item>,
    pub flash_timer_sec:f32,
    pub flash_timer_start:f32,
    pub flash_max:f32,
    pub pokemon_cards:Counter,
    pub inventory:HashMap<Textures, f32>
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

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Item {
    pub pickup:bool
}

impl Door {
    pub fn open_door(&mut self) {
        self.open = true;
        self.close_timer_sec = 5.0; // close after 5 sec
    }
}