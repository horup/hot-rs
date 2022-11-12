use crate::Id;

#[derive(Debug, Clone)]
pub enum Command {
    Restart,
    DefineTexture {
        handle:u32,
        path:String
    },
    DefineSound {
        handle:u32,
        path:String
    },
    LoadMap {
        map_path:String
    },
    DespawnEntity {
        id:Id
    }
}