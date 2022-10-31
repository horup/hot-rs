use glam::IVec2;

use crate::EntityKey;


#[derive(Debug, Clone)]
pub enum Command {
    Restart,
    DefineTexture {
        handle:u32,
        path:String
    },
    LoadMap {
        map_path:String
    },
    ContactEntity {
        entity:EntityKey,
        other:EntityKey
    },
    ContactTile {
        entity:EntityKey,
        tile:IVec2
    }
}