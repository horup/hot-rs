
#[derive(Debug, Clone)]
pub enum Command {
    Restart,
    DefineTexture {
        handle:u32,
        src:String
    }
}