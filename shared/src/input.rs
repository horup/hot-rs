use glam::Vec2;

#[derive(Default)]
pub struct PlayerInput {
    pub dir: Vec2,
    pub action: bool,
    pub mouse_pos_screen: Vec2,
    pub mouse_pos_world: Vec2,
    pub mouse_left_down: bool,
    pub mouse_right_down: bool,
    pub mouse_left_pressed: bool,
    pub mouse_right_pressed: bool,
}
