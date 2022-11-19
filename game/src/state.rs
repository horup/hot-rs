use std::{collections::HashMap, ops::{Deref, DerefMut}};

use shared::{Id, Camera, Components, World, glam::{IVec2, Vec3}};
use serde::{Serialize, Deserialize};

use crate::{Images, Critter};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct State {
    pub world:World,
    pub camera:Camera,
    pub player:Option<Id>,
    pub walkers:Components<Walker>,
    pub doors:Components<Door>,
    pub items:Components<Item>,
    pub critters:Components<Critter>,
    pub flash:Flash,
    pub pokemon_cards:Counter,
    pub inventory:HashMap<Images, f32>,
    pub chosen_character:Option<Images>,
    pub pause:bool,
    pub start_pos:Vec3,
    pub won:bool,
    pub lost:bool
}

impl State {
    pub fn new(world:World) -> Self {
        Self {
            world,
            ..Default::default()
        }
    }
}

impl Deref for State {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Counter {
    pub current:f32,
    pub total:f32
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Flash {
    pub flash_timer_sec:f32,
    pub flash_timer_start:f32,
    pub flash_max:f32,
}

impl Flash {
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
    pub key:Option<Images>
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Item {
    pub pickup:bool,
    pub pickup_sound:Option<u32>
}

impl Door {
    pub fn open_door(&mut self) {
        self.open = true;
        self.close_timer_sec = 5.0; // close after 5 sec
    }
}