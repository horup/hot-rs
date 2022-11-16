mod images;
pub use images::*;

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

pub mod sounds;

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
       
    }
  
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self.state).unwrap()
    } 

    fn deserialize(&mut self, bytes:&[u8]) {
        self.state = bincode::deserialize(bytes).unwrap();
    }
}
  
impl MyGame {
    fn process_events(&mut self, engine: &mut dyn Context) {
        for event in engine.events().iter() {
            match event {
                Event::MapReady { map } => {
                    self.start(engine, map);
                },
                Event::Start {} => {
                    engine.push_command(Command::LoadMap { map_path: "assets/maps/test.map".into() });
                }
                Event::Restart {  } => {
                    engine.push_command(Command::LoadMap { map_path: "assets/maps/test.map".into() });
                },
            }
        }
    }
}

#[no_mangle]
pub fn create(engine:&mut dyn Context) -> Box<dyn Game> {
    init(engine);
    Box::new(MyGame::default())
}