use glam::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Entity {
    pub pos:Vec3,
    pub dir:f32,
    pub texture:u32
}