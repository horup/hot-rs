use serde::{Serialize, Deserialize};
use shared::glam::{Vec2, Vec3};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Critter {
    pub last_known_pos:Option<Vec3>,
    pub can_see_player:bool,
    pub dir:Vec2
}