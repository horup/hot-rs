mod textures;
pub use textures::*;

mod init;
pub use init::*;

mod state;
pub use state::*;

mod start;
pub use start::*;

use shared::{*, glam::Vec2};

mod input;
pub use input::*;

mod update;
pub use update::*;

mod draw;
pub use draw::*;

#[derive(Default)]
pub struct MyGame {
    pub state:State,
    pub dir:Vec2
}
 
impl Game for MyGame {
    fn tick(&mut self, ctx:&mut dyn Context) {
        self.poll_input(ctx); 
        self.process_events(ctx);
        self.update(ctx);
        self.draw(ctx);  
        ctx.draw_world(&self.state.camera); 
    }
  
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self.state).unwrap()
    }

    fn deserialize(&mut self, bytes:&[u8]) {
        dbg!("deserialize start");
        self.state = bincode::deserialize(bytes).unwrap();
        dbg!("deserialize");
    }

    fn init(&mut self, engine:&mut dyn Context) {
        init(engine);
        if self.state.loaded == false {
            engine.push_command(Command::LoadMap { map_path: "assets/maps/test.map".into() });
            self.state.loaded = true; 
        }
    } 
}

impl MyGame {
    fn process_events(&mut self, engine: &mut dyn Context) {
        for event in engine.events().iter() {
            match event {
                Event::MapLoaded {  } => {
                    self.start(engine);
                },
                _=>{}
            }
        }
    }
}

#[no_mangle]
pub fn create(_engine:&mut dyn Context) -> Box<dyn Game> {
    Box::new(MyGame::default())
}