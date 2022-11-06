use crate::Engine;

pub trait Game {
    fn tick(&mut self, engine:&mut dyn Engine);
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, vec:&[u8]);
}