use context::{EntityKey, slotmap::SecondaryMap};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct State {
    pub player:Option<EntityKey>,
    pub walkers:SecondaryMap<EntityKey, Walker>,

}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Walker {
    pub walker:f32,
}

pub static mut STATE:Option<State> = None; 