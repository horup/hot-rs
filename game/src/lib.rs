use context::{Command, Context};

pub enum Textures {
    Floor = 4,
    Player = 1,
    Wall = 5,
    Piggy = 2,
    Key = 3,
}
impl Into<u32> for Textures {
    fn into(self) -> u32 {
        self as u32
    }
}



#[no_mangle]
pub fn init(ctx: &mut Context) {
    ctx.edit_mode = true;
    ctx.debug = true;

    ctx.define_texture(Textures::Player, "assets/textures/guy3.png");
    ctx.define_texture(Textures::Piggy, "assets/textures/piggy4.png");
    ctx.define_texture(Textures::Key, "assets/textures/key3.png");

    ctx.define_texture(Textures::Floor, "assets/textures/floor2.png");
    ctx.define_texture(Textures::Wall, "assets/textures/wall2.png");

    let edit = &mut ctx.edit;
    edit.entities = [Textures::Player.into(), Textures::Player.into(), Textures::Key.into()].to_vec().into();
    edit.tiles = [Textures::Wall.into(), Textures::Floor.into()].into();
}

#[no_mangle]
pub fn update(engine: &mut Context) {
    if engine.input.action {
        engine.commands.push(Command::FlashScreen {});
    }
}
