use glam::{Vec3, IVec2};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Sprite {
    pub pos:Vec3,
    pub vel:Vec3,
    pub dir:f32,
    pub img:u32,
    pub flip_x:bool,
    pub radius:f32,
    pub no_clip:bool,
    pub hidden:bool
}

impl Sprite {
    pub fn cell(&self) -> IVec2 {
        // FIXME: not 100% correct if x or y is negativ
        IVec2::new(self.pos.x as i32, self.pos.y as i32)
    }
}