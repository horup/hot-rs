use shared::*;

pub struct MyGame {

}

impl Game for MyGame {
    fn tick(&mut self, engine:&mut dyn Engine) {
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    fn deserialize(&mut self, vec:&[u8]) {
    }
}


#[no_mangle]
pub fn init(engine:&mut dyn Engine) -> Box<dyn Game> {

    
    Box::new(MyGame {

    })
}

/*mod state;
use context::{Context, slotmap::SlotMap, Id, Entity};
use serde::{Serialize, Deserialize};
pub use state::*;

use crate::STATE;

#[derive(Serialize, Deserialize)]
struct S {
    pub entities:SlotMap<Id, Entity>,
    pub state:State
}

#[no_mangle]
pub fn serialize(ctx:&mut Context) -> Vec<u8> {
    if let Some(state) = unsafe { STATE.clone()} {
        let v = bincode::serialize(&S {
            entities:ctx.entities.clone(),
            state
        }).unwrap();
        return v;
    }
 
    Vec::new()
}

#[no_mangle]
pub fn deserialize(ctx:&mut Context, state:&Vec<u8>) {
    let s:S = bincode::deserialize(state).unwrap(); 
    ctx.entities = s.entities;
    unsafe {STATE = Some(s.state)}
}
 
mod init;
pub use init::*;

mod start;
pub use start::*;

mod update;
pub use update::*;

mod draw;
pub use draw::*;

mod post_update;
pub use post_update::*;*/