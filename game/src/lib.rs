mod textures;
pub use textures::*;

mod init;
pub use init::*;

mod state;
pub use state::*;

mod start;
pub use start::*;

use shared::{*, glam::Vec2};

#[derive(Default)]
pub struct MyGame {
    pub state:State
}

impl Game for MyGame {
    fn tick(&mut self, engine:&mut dyn Context) {
        for event in engine.events().iter() {
            match event {
                Event::MapLoaded {  } => {
                    self.start(engine)
                },
            }
        }
        let camera = Camera {
            pos: Vec2::new(5.0, 0.0),
            zoom: 32.0,
        };
        engine.draw_world(&camera);
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    fn deserialize(&mut self, _vec:&[u8]) {
    }

    fn init(&mut self, engine:&mut dyn Context) {
        init(engine);
    }
}


#[no_mangle]
pub fn create(_engine:&mut dyn Context) -> Box<dyn Game> {
    Box::new(MyGame::default())
}