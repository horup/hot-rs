use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize};

use crate::{Grid, Map, Color};


#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub img:Option<u32>,
    pub clips:bool,
    pub hidden:bool,
    pub diffuse:Color
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Tiles {
    grid:Grid<Tile>
}

impl Tiles {
    pub fn clear(&mut self) {
        self.grid = Grid::new(self.grid.size());
    }
}

impl From<&Map> for Tiles {
    fn from(map: &Map) -> Self {
        let s = map.grid.size();
        let mut grid:Grid<Tile> = Grid::new(s);
        for y in 0..s {
            for x in 0..s {
                let tile = map.grid.get(x as i32, y as i32).unwrap();
                grid.get_mut(x as i32, y as i32).unwrap().img = tile.tile;
                grid.get_mut(x as i32, y as i32).unwrap().clips = tile.blocks;
            }
        }

        Tiles {
            grid
        }
    }
}

impl Deref for Tiles {
    type Target = Grid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for Tiles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
    
}