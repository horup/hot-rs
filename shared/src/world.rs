use serde::{Serialize, Deserialize};
use crate::{Sprites, Tiles};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct World {
    pub sprites:Sprites,
    pub tiles:Tiles
}

impl World {
    pub fn clear(&mut self) {
        self.sprites.clear();
        self.tiles.clear();
    }
}