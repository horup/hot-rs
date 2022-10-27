mod state;
pub use state::*;

use crate::STATE;

#[no_mangle]
pub fn serialize() -> Vec<u8> {
    if let Some(state) = unsafe { STATE.as_ref()} {
        let v = bincode::serialize(state).unwrap();
        return v;
    }

    Vec::new()
}

#[no_mangle]
pub fn deserialize(state:&Vec<u8>) {
    unsafe {STATE = Some(bincode::deserialize::<State>(state).unwrap())}
}
 
 


mod init;
pub use init::*;

mod start;
pub use start::*;

mod update;
pub use update::*;