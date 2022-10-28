use context::EntityKey;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub player:Option<EntityKey>,
}

pub static mut STATE:Option<State> = None; 