use glam::IVec2;

use crate::Id;


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
        entity:Id,
        other:Id
    },
    ContactTile {
        entity:Id,
        tile:IVec2
    }
}