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
        Vec::new()
    }

    fn deserialize(&mut self, _vec:&[u8]) {
    }

    fn init(&mut self, engine:&mut dyn Context) {
        init(engine);
    }
}

impl MyGame {
    fn process_events(&mut self, engine: &mut dyn Context) {
        for event in engine.events().iter() {
            match event {
                Event::MapLoaded {  } => {
                    self.start(engine);
                },
            }
        }
    }
}

#[no_mangle]
pub fn create(_engine:&mut dyn Context) -> Box<dyn Game> {
    Box::new(MyGame::default())
}