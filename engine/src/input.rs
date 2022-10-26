use context::{Command, PlayerInput, glam::Vec2, Tool};
use macroquad::prelude::{is_key_pressed, KeyCode, is_key_down, mouse_position, is_mouse_button_down, MouseButton, is_mouse_button_pressed, mouse_wheel};

use crate::Engine;

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
        return None;
    }

    pub fn edit_input(&mut self) {
        let before = self.ctx.map.clone();
        if is_key_pressed(KeyCode::B) {
            self.ctx.edit.blocks = !self.ctx.edit.blocks;
        }

        let (_, mw_y) = mouse_wheel();
        self.ctx.edit_camera.zoom -= mw_y / 100.0;
        if self.ctx.edit_camera.zoom < 2.0 {
            self.ctx.edit_camera.zoom = 2.0;
        }


        let speed = self.ctx.edit_camera.zoom * self.ctx.dt;
        self.ctx.edit_camera.pos += self.ctx.input.dir * speed;


        if is_key_pressed(KeyCode::F5) {
            self.save_map_to_file();
        }
        if is_key_pressed(KeyCode::F6) {
            self.load_map_from_file();
        }

        if is_key_pressed(KeyCode::E) {
            self.ctx.edit.tool = Tool::Entity;
        }
        if is_key_pressed(KeyCode::T) {
            self.ctx.edit.tool = Tool::Tile;
        }

        if let Some(num) = self.num() {
            let t = num as u32;
            match self.ctx.edit.tool {
                Tool::Tile => self.ctx.edit.selected_tile = t,
                Tool::Entity => self.ctx.edit.selected_entity = t,
            }
        }

        if self.ctx.over_ui == false {
            let cell = self.ctx.input.mouse_pos_world.floor();
            if let Some(cell) = self.ctx.map.grid.get_mut(cell.x as i32, cell.y as i32) {
                if is_mouse_button_down(MouseButton::Left) {
                    match self.ctx.edit.tool {
                        Tool::Tile => {
                            cell.tile = Some(self.ctx.edit.selected_tile);
                            cell.blocks = self.ctx.edit.blocks;
                        },
                        Tool::Entity => {
                            cell.entity = Some(self.ctx.edit.selected_entity);
                        },
                    }
                }
                else if is_mouse_button_down(MouseButton::Right) {
                    match self.ctx.edit.tool {
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

        let after = self.ctx.map.clone();
        if after != before {
            
        }
    
    }

    pub fn input(&mut self) {
        if is_key_pressed(KeyCode::Tab) {
            self.ctx.edit_mode = !self.ctx.edit_mode;
        }

        if is_key_pressed(KeyCode::F1) {
            self.ctx.commands.push(Command::Restart);
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
        self.ctx.input = PlayerInput { 
            dir:Vec2::new(x, y), 
            action: action,
            mouse_pos_screen:m,
            mouse_pos_world:self.to_world(m),
            mouse_left_down:is_mouse_button_down(MouseButton::Left),
            mouse_right_down:is_mouse_button_down(MouseButton::Right),
            mouse_left_pressed:is_mouse_button_pressed(MouseButton::Left),
            mouse_right_pressed:is_mouse_button_pressed(MouseButton::Right)
        };
        
        if self.ctx.edit_mode {
            self.edit_input();
        }
    }
}