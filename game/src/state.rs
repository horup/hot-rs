use shared::{Id, slotmap::SecondaryMap, Camera};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct State {
    pub camera:Camera,
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