use shared::{glam::Vec3, Context, Entity, IgnoreColissions, Tiles};
use num_enum::TryFromPrimitive;
use crate::{Textures, Walker, Door, MyGame, Item};

impl MyGame {
    pub fn start(&mut self, engine:&mut dyn Context) {
        engine.clear();
        let map = engine.map().clone();

        let w = Tiles::from(&map);
        *engine.tiles_mut() = w;

        let state = &mut self.state;
        engine.map().clone().grid.for_each_mut(|cell, x, y| {
            let pos = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            if let Some(entity) = cell.entity {
                if let Ok(entity) = Textures::try_from_primitive(entity) {
                    match entity {
                        Textures::Viktor => {
                            dbg!("Spawning Player");
                            let player_entity = engine.entities_mut().spawn_entity(Entity {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.25,
                                ..Default::default()
                            });
    
                            state.player = Some(player_entity);
                            state.walkers.attach(player_entity, Walker::default());
                        }
                        Textures::PokemonCard => {
                            let card = engine.entities_mut().spawn_entity(Entity {
                                pos, 
                                texture: entity.into(),
                                radius: 0.25,
                                ignore_collisions: IgnoreColissions::WithEntities,
                                ..Default::default()
                            });

                            state.items.attach(card, Item {
                                pickup:true
                            });
                        },
                        Textures::WhiteDoor
                        | Textures::WhiteDoorSide
                        | Textures::BlueDoor
                        | Textures::GoldDoor => {
                            let door = engine.entities_mut().spawn_entity(Entity {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.5,
                                ..Default::default()
                            });
    
                            state.doors.attach(door, Door::default());
                        }
                        _ => {
                            engine.entities_mut().spawn_entity(Entity {
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
