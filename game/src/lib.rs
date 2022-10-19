use context::{Entity, Context, Command};

static PLAYER_TEXTURE:u32 = 1;
static PIGGY_TEXTURE:u32 = 2;
static KEY_TEXTURE:u32 = 3;
static FLOOR_TEXTURE:u32 = 4;
static WALL_TEXTURE:u32 = 5;

#[no_mangle]
pub fn init(ctx:&mut Context) {
    ctx.debug = true;
    ctx.define_texture(PLAYER_TEXTURE, "assets/textures/guy3.png");
    ctx.define_texture(PIGGY_TEXTURE, "assets/textures/piggy4.png");
    ctx.define_texture(KEY_TEXTURE, "assets/textures/key3.png");

    ctx.define_texture(FLOOR_TEXTURE, "assets/textures/floor2.png");
    ctx.define_texture(WALL_TEXTURE, "assets/textures/wall2.png");

    ctx.state.tilemap.get_mut(0, 0).unwrap().texture = WALL_TEXTURE;
    ctx.state.tilemap.get_mut(0, 1).unwrap().texture = FLOOR_TEXTURE;
}


#[no_mangle]
pub fn update(engine:&mut Context) {
    engine.state.iterations += 1;
    let speed = 0.1;
    engine.state.camera.zoom = 16.0;
    engine.state.camera.pos += engine.player_input.dir * speed;

    if engine.player_input.action {
        
        engine.commands.push(Command::FlashScreen {  });
    }

}
