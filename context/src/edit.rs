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

#[derive(Default, Copy, Clone)]
pub struct Edit {
    pub tile_texture:u32,
    pub entity_texture:u32,
    pub tool:Tool
}