use context::{Context, Command};
use crate::state_mut;

#[no_mangle]
pub fn post_update(context:&mut Context) {
    let state = state_mut();
    for c in context.commands.iter() {
        match c {
            Command::ContactEntity { entity: _, other } => {
                if let Some(door) = state.doors.get_mut(*other) {
                    door.open_door();
                }
            },
            Command::ContactTile { entity: _, tile: _ } => {
            },
            _=>{}
        }
    }
}