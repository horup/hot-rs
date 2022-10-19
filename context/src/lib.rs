
#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub texture:u32
}

#[derive(Debug, Clone)]
pub struct Tilemap {
    size:usize,
    tiles:Vec<Tile>
}

impl Default for Tilemap {
    fn default() -> Self {
        let size = 64;
        Self { 
            size,
            tiles:vec![Tile::default();size * size]
         }
    }
}

impl Tilemap {
    pub fn size(&self)->usize {
        return self.size;
    }

    pub fn get_mut(&mut self, x:i32, y:i32) -> Option<&mut Tile> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.tiles.get_mut(index);
        }

        return None;
    }

    pub fn get(&self,x:i32, y:i32) -> Option<&Tile> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.tiles.get(index);
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
    pub tilemap:Tilemap
}

#[derive(Default)]
pub struct PlayerInput {
    pub dir:Vec2,
    pub action:bool
}

#[derive(Default)]
pub struct Context {
    pub state:State,
    pub commands:Vec<Command>,
    pub player_input:PlayerInput,
    pub debug:bool
}

impl Context {
    pub fn define_texture(&mut self, handle:u32, src:&str) {
        self.commands.push(Command::DefineTexture { handle: handle, path: src.into() })
    }
}


mod command;
pub use command::*;
use glam::Vec2;

pub use glam;