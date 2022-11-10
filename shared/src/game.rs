use crate::Context;

pub trait Game {
    fn tick(&mut self, ctx:&mut dyn Context);
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, vec:&[u8]);
}