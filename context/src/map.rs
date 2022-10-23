use enumflags2::{bitflags, BitFlags};
use serde::{Serialize, Deserialize};

use crate::Grid;

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TileFlags {
    Wall
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct MapCell {
    pub tile:Option<u32>,
    pub entity:Option<u32>,
    pub blocks:bool,
    pub tile_flags:BitFlags<TileFlags>
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Map {
    pub version:u8,
    pub grid:Grid<MapCell>
}