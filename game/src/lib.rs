mod state;
use context::{Context, slotmap::SlotMap, EntityKey, Entity};
use serde::{Serialize, Deserialize};
pub use state::*;

use crate::STATE;

#[derive(Serialize, Deserialize)]
struct S {
    pub entities:SlotMap<EntityKey, Entity>,
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