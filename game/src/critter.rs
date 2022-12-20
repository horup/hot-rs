use serde::{Serialize, Deserialize};
use shared::glam::Vec2;

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Critter {
    pub dir:Vec2
}