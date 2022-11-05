use crate::Engine;

pub trait Game {
    fn init(&mut self, engine:&mut dyn Engine);
    fn tick(&mut self, engine:&mut dyn Engine);
}