use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize};

use crate::{Grid, Map};


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub img:Option<u32>,
    pub clips:bool,
    pub hidden:bool
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct World {
    grid:Grid<Tile>
}

impl World {
    pub fn clear(&mut self) {
        self.grid = Grid::new(self.grid.size());
    }
}

impl From<&Map> for World {
    fn from(map: &Map) -> Self {
        let s = map.grid.size();
        let mut grid:Grid<Tile> = Grid::new(s);
        for y in 0..s {
            for x in 0..s {
                let tile = map.grid.get(x as i32, y as i32).unwrap();
                grid.get_mut(x as i32, y as i32).unwrap().img = tile.tile;
             
            }
        }

        World {
            grid
        }
    }
}

impl Deref for World {
    type Target = Grid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
    
}