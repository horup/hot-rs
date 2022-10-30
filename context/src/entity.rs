use glam::{Vec3, IVec2};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IgnoreColissions {
    None,
    WithEntities
}
impl Default for IgnoreColissions {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Entity {
    pub pos:Vec3,
    pub vel:Vec3,
    pub dir:f32,
    pub texture:u32,
    pub flip_x:bool,
    pub radius:f32,
    pub ignore_collisions:IgnoreColissions
}

impl Entity {
    pub fn cell(&self) -> IVec2 {
        // FIXME: not 100% correct if x or y is negativ
        IVec2::new(self.pos.x as i32, self.pos.y as i32)
    }
}