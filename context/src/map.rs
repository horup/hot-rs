use crate::Grid;

#[derive(Clone, Copy, Default)]
pub struct MapCell {
    pub tile:Option<u32>,
    pub entity:Option<u32>,
    pub blocks:bool
}

#[derive(Clone, Default)]
pub struct Map {
    pub grid:Grid<MapCell>
}