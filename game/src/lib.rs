

use context::{Command, Context, Mode};

static PLAYER_TEXTURE: u32 = 1;
static PIGGY_TEXTURE: u32 = 2;
static KEY_TEXTURE: u32 = 3;
static FLOOR_TEXTURE: u32 = 4;
static WALL_TEXTURE: u32 = 5;



#[no_mangle]
pub fn init(ctx: &mut Context) {
    ctx.mode = Mode::Edit;
    ctx.debug = true;
    ctx.define_texture(PLAYER_TEXTURE as u32, "assets/textures/guy3.png");
    ctx.define_texture(PIGGY_TEXTURE, "assets/textures/piggy4.png");
    ctx.define_texture(KEY_TEXTURE, "assets/textures/key3.png");

    ctx.define_texture(FLOOR_TEXTURE, "assets/textures/floor2.png");
    ctx.define_texture(WALL_TEXTURE, "assets/textures/wall2.png");

    let edit = &mut ctx.edit;
    edit.entities = [PLAYER_TEXTURE, PIGGY_TEXTURE, KEY_TEXTURE].into();
    edit.tiles = [FLOOR_TEXTURE, WALL_TEXTURE].into();



    /* let tilemap = &mut ctx.state.tilemap;
    for y in 0..tilemap.size() {
        for x in 0..tilemap.size() {
            tilemap.get_mut(x as i32, y as i32).unwrap().texture = FLOOR_TEXTURE;
        }
    }

    for y in 0..1 {
        for x in 0..tilemap.size() {
            tilemap.get_mut(x as i32, y as i32).unwrap().texture = WALL_TEXTURE;
        }
    }

    for y in 0..6 {
        for x in 0..1 {
            tilemap.get_mut(x as i32, y as i32).unwrap().texture = WALL_TEXTURE;
        }
    }

    for y in 6..7 {
        for x in 0..10 {
            tilemap.get_mut(x as i32, y as i32).unwrap().texture = WALL_TEXTURE;
        }
    }*/
}

#[no_mangle]
pub fn update(engine: &mut Context) {
    engine.state.iterations += 1;
    let speed = 0.1;
    engine.state.camera.zoom = 16.0;
    engine.state.camera.pos += engine.input.dir * speed;

    if engine.input.action {
        engine.commands.push(Command::FlashScreen {});
    }
}
