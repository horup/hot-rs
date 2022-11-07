use shared::{glam::Vec3, Context, Entity, IgnoreColissions};
use num_enum::TryFromPrimitive;
use crate::{Textures, Walker, Door, MyGame};

impl MyGame {
    pub fn start(&mut self, engine:&mut dyn Context) {
        engine.clear();
        let state = &mut self.state;
        engine.map().clone().grid.for_each_mut(|cell, x, y| {
            if let Some(entity) = cell.entity {
                if let Ok(entity) = Textures::try_from_primitive(entity) {
                    match entity {
                        Textures::William => {
                            dbg!("Spawning Player");
                            let player_entity = engine.spawn_entity(Entity {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.25,
                                ..Default::default()
                            });
                            
    
                            state.player = Some(player_entity);
                            state.walkers.insert(player_entity, Walker::default());
                        }
                        Textures::WhiteDoor
                        | Textures::WhiteDoorSide
                        | Textures::BlueDoor
                        | Textures::GoldDoor => {
                            let door = engine.spawn_entity(Entity {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.5,
                                ..Default::default()
                            });
    
                            state.doors.insert(door, Door::default());
                        }
                        _ => {
                            engine.spawn_entity(Entity {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.5,
                                ignore_collisions: IgnoreColissions::WithEntities,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        })
    }
}
