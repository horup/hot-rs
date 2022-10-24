use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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