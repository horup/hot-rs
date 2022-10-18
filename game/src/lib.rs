use context::{Entity, Context, Command};

static PLAYER_TEXTURE:u32 = 0;
static PIGGY_TEXTURE:u32 = 1;
static KEY_TEXTURE:u32 = 2;

#[no_mangle]
pub fn init(engine:&mut Context) {
    engine.define_texture(PLAYER_TEXTURE, "assets/textures/player.png");
    engine.define_texture(PIGGY_TEXTURE, "assets/textures/piggy.png");
    engine.define_texture(KEY_TEXTURE, "assets/textures/key.png");

    engine.state.entities.push(Entity { 
        x:100.0,
        y:100.0, 
        texture:PLAYER_TEXTURE
    });


    engine.state.entities.push(Entity { 
        x:300.0,
        y:500.0, 
        texture:PIGGY_TEXTURE
    });

    println!("Initialized state to {:?}", engine.state);
}


#[no_mangle]
pub fn update(engine:&mut Context) {
    engine.state.iterations += 1;

    for e in engine.state.entities.iter_mut() {
        let speed = 1.0;
        e.x += engine.player_input.x * speed;
        e.y += engine.player_input.y * speed;
    }

    if engine.player_input.action {
        engine.commands.push(Command::FlashScreen {  });
    }

}
