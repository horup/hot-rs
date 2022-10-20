
#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub texture:u32
}

#[derive(Debug, Clone)]
pub struct Grid<T:Clone + Default> {
    size:usize,
    cells:Vec<T>
}

impl<T> Default for Grid<T> where T:Default+Clone {
    fn default() -> Self {
        let size = 64;
        Self { 
            size,
            cells:vec![T::default();size * size]
         }
    }
}

impl<T> Grid<T> where T:Default+Clone {
    pub fn size(&self)->usize {
        return self.size;
    }

    pub fn get_mut(&mut self, x:i32, y:i32) -> Option<&mut T> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.cells.get_mut(index);
        }

        return None;
    }

    pub fn get(&self,x:i32, y:i32) -> Option<&T> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.cells.get(index);
        }

        return None;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub x:f32,
    pub y:f32,
    pub texture:u32
}

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

#[derive(Default, Debug, Clone)]
pub struct State {
    pub camera:Camera,
    pub iterations:u64,
    pub entities:Vec<Entity>,
    pub tilemap:Grid<Tile>
}

#[derive(Default)]
pub struct PlayerInput {
    pub dir:Vec2,
    pub action:bool,
    pub mouse_pos_screen:Vec2,
    pub mouse_pos_world:Vec2,
    pub mouse_left_down:bool,
    pub mouse_right_down:bool,
    pub mouse_left_pressed:bool,
    pub mouse_right_pressed:bool
}


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Play,
    Edit
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Play
    }
}

#[derive(Default)]
pub struct Context {
    pub mode:Mode,
    pub map:Map,
    pub state:State,
    pub commands:Vec<Command>,
    pub input:PlayerInput,
    pub debug:bool,
    pub edit:Edit
}

impl Context {
    pub fn define_texture(&mut self, handle:u32, src:&str) {
        self.commands.push(Command::DefineTexture { handle: handle, path: src.into() })
    }
}


mod command;
pub use command::*;
mod map;
pub use map::*;
mod edit;
pub use edit::*;

use glam::Vec2;

pub use glam;