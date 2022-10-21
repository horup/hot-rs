use serde::{Serialize, Deserialize};

use crate::Grid;

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct MapCell {
    pub tile:Option<u32>,
    pub entity:Option<u32>,
    pub blocks:bool
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Map {
    pub version:u8,
    pub grid:Grid<MapCell>
}