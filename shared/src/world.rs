use serde::{Serialize, Deserialize};
use crate::{Sprites, Tiles};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct World {
    pub sprites:Sprites,
    pub tiles:Tiles
}