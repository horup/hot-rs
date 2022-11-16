use crate::Map;

pub enum Event {
    Start,
    MapReady {
        map:Map
    },
    Restart {
        
    }
}