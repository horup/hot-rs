use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Entity {
    pub pos:Vec3,
    pub dir:f32,
    pub texture:u32
}