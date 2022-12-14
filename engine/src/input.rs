use shared::{glam::Vec2, Event};
use macroquad::{prelude::{is_key_pressed, KeyCode, is_key_down, mouse_position, is_mouse_button_down, MouseButton, is_mouse_button_pressed, mouse_wheel}, time::get_frame_time};

use crate::{Engine, Tool};

#[derive(Default)]
pub struct EditInput {
    pub dir: Vec2,
    pub action: bool,
    pub mouse_pos_screen: Vec2,
    pub mouse_pos_world: Vec2,
    pub mouse_left_down: bool,
    pub mouse_right_down: bool,
    pub mouse_left_pressed: bool,
    pub mouse_right_pressed: bool,
}


impl Engine {
    pub fn num(&self) -> Option<u8> {
        if is_key_pressed(KeyCode::Key0) { return Some(0); }
        if is_key_pressed(KeyCode::Key1) { return Some(1); }
        if is_key_pressed(KeyCode::Key2) { return Some(2); }
        if is_key_pressed(KeyCode::Key3) { return Some(3); }
        if is_key_pressed(KeyCode::Key4) { return Some(4); }
        if is_key_pressed(KeyCode::Key5) { return Some(5); }
        if is_key_pressed(KeyCode::Key6) { return Some(6); }
        if is_key_pressed(KeyCode::Key7) { return Some(7); }
        if is_key_pressed(KeyCode::Key8) { return Some(8); }
        if is_key_pressed(KeyCode::Key9) { return Some(9); }
        None
    }

    pub fn edit_input(&mut self) {
        let before = self.map.clone();
        if is_key_pressed(KeyCode::B) {
            self.edit.blocks = !self.edit.blocks;
        }

        let (_, mw_y) = mouse_wheel();
        self.edit_camera.zoom -= mw_y / 100.0;
        if self.edit_camera.zoom < 2.0 {
            self.edit_camera.zoom = 2.0;
        }


        let speed = self.edit_camera.zoom * get_frame_time();
        self.edit_camera.pos += self.edit_input.dir * speed;


        if is_key_pressed(KeyCode::F5) {
            self.save_map_to_file();
        }
        if is_key_pressed(KeyCode::F6) {
            self.load_map_from_file();
        }

        if is_key_pressed(KeyCode::E) {
            self.edit.tool = Tool::Entity;
        }
        if is_key_pressed(KeyCode::T) {
            self.edit.tool = Tool::Tile;
        }

        if let Some(num) = self.num() {
            let t = num as u32;
            match self.edit.tool {
                Tool::Tile => self.edit.selected_tile = t,
                Tool::Entity => self.edit.selected_entity = t,
            }
        }

        if !self.over_ui {
            let cell = self.edit_input.mouse_pos_world.floor();
            if let Some(cell) = self.map.grid.get_mut(cell.x as i32, cell.y as i32) {
                if is_mouse_button_down(MouseButton::Left) {
                    match self.edit.tool {
                        Tool::Tile => {
                            cell.tile = Some(self.edit.selected_tile);
                            cell.blocks = self.edit.blocks;
                        },
                        Tool::Entity => {
                            cell.entity = Some(self.edit.selected_entity);
                        },
                    }
                }
                else if is_mouse_button_down(MouseButton::Right) {
                    match self.edit.tool {
                        Tool::Tile => {
                            cell.tile = None;
                            cell.blocks = false;
                        },
                        Tool::Entity => {
                            cell.entity = None;
                        },
                    }
                }
            }
        }

        let after = self.map.clone();
        if after != before {
            self.events.push(Event::MapReady { map:after });
        }
    
    }

    pub fn input(&mut self) {
        if is_key_pressed(KeyCode::Tab) {
            self.edit_mode = !self.edit_mode;
        }

        if is_key_pressed(KeyCode::F1) {
            self.events.push(Event::Restart {});
        }
    
        let mut x = 0.0;
        let mut y = 0.0;
        let action = is_key_down(KeyCode::Space);
    
        if is_key_down(KeyCode::A) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            x += 1.0;
        }
        if is_key_down(KeyCode::W) {
            y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            y += 1.0;
        }
    
        let m = Vec2::new(mouse_position().0, mouse_position().1);
        self.edit_input = EditInput { 
            dir:Vec2::new(x, y), 
            action,
            mouse_pos_screen:m,
            mouse_pos_world:self.to_world(m),
            mouse_left_down:is_mouse_button_down(MouseButton::Left),
            mouse_right_down:is_mouse_button_down(MouseButton::Right),
            mouse_left_pressed:is_mouse_button_pressed(MouseButton::Left),
            mouse_right_pressed:is_mouse_button_pressed(MouseButton::Right)
        };
        
        if self.edit_mode {
            self.edit_input();
        }
    }
}