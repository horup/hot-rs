use serde::{Serialize, Deserialize};
use crate::Grid;

#[derive(Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapCell {
    #[serde(default)]
    pub tile:Option<u32>,
    #[serde(default)]
    pub entity:Option<u32>,
    #[serde(default)]
    pub blocks:bool
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Map {
    pub version:u8,
    pub grid:Grid<MapCell>
}