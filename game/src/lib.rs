use context::{Entity, Context};
#[no_mangle]
pub fn init(engine:&mut Context) {
    engine.state.entities.push(Entity {
        x:100.0,
        y:100.0
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

}
