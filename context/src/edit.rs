#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tool {
    Tile,
    Entity
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Tile
    }
}

#[derive(Default, Clone)]
pub struct Edit {
    pub tiles:Vec<u32>,
    pub entities:Vec<u32>,
    pub selected_tile:u32,
    pub selected_entity:u32,
    pub tool:Tool,
    pub blocks:bool
}