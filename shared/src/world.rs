use serde::{Serialize, Deserialize};
use crate::{Entities, Tiles};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct World {
    pub entities:Entities,
    pub tiles:Tiles
}

impl World {
    pub fn clear(&mut self) {
        self.entities.clear();
        self.tiles.clear();
    }
}