
#[derive(Debug, Clone)]
pub enum Command {
    Restart,
    DefineTexture {
        handle:u32,
        path:String
    },
    FlashScreen {
    },
    LoadMap {
        map_path:String
    }
}