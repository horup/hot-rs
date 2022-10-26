use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Camera {
    pub pos:Vec2,
    pub zoom:f32
}
impl Default for Camera {
    fn default() -> Self {
        Self { pos: Default::default(), zoom: 1.0 / 8.0 }
    }
}