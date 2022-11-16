use shared::{glam::Vec3, Context, Entity, IgnoreColissions, Tiles, Color, World, Map};
use num_enum::TryFromPrimitive;
use crate::{Images, Walker, Door, MyGame, Item, sounds};

impl MyGame {
    pub fn start(&mut self, _engine:&mut dyn Context, map:&Map) {
        self.state.world.clear();
        let mut tiles = Tiles::from(map);
        tiles.for_each_mut(|t,_,_| {
            t.diffuse = Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            };
        });
        
        self.state.world = World {
            entities: Default::default(),
            tiles,
        }; 

        let state = &mut self.state;
        map.grid.for_each(|cell, x, y| {
            let pos = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            if let Some(entity) = cell.entity {
                if let Ok(entity) = Images::try_from_primitive(entity) {
                    match entity {
                        Images::Viktor => {
                            dbg!("Spawning Player");
                            let player_entity = state.world.entities.spawn_entity(Entity {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.25,
                                ..Default::default()
                            });
    
                            state.player = Some(player_entity);
                            state.walkers.attach(player_entity, Walker::default());
                        }
                        Images::PokemonCard => {
                            let card = state.world.entities.spawn_entity(Entity {
                                pos, 
                                texture: entity.into(),
                                radius: 0.25,
                                ignore_collisions: IgnoreColissions::WithEntities,
                                ..Default::default()
                            });

                            state.items.attach(card, Item {
                                pickup:true,
                                ..Default::default()
                            });

                            state.pokemon_cards.total += 1.0;
                        },
                        Images::GoldKey 
                        | Images::BlueKey => {
                            let key = state.world.entities.spawn_entity(Entity {
                                pos, 
                                texture: entity.into(),
                                radius: 0.25,
                                ignore_collisions: IgnoreColissions::WithEntities,
                                ..Default::default()
                            });

                            state.items.attach(key, Item {
                                pickup:true,
                                pickup_sound:Some(sounds::PICKUP_KEY)
                            });
                        },
                        Images::WhiteDoor
                        | Images::WhiteDoorSide
                        | Images::BlueDoor
                        | Images::GoldDoor => {
                            let door = state.world.entities.spawn_entity(Entity {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.5,
                                ..Default::default()
                            });

                            let mut key = None;
                            if entity == Images::BlueDoor {
                                key = Some(Images::BlueKey);
                            } else if entity == Images::GoldDoor {
                                key = Some(Images::GoldKey);
                            }
    
                            state.doors.attach(door, Door {
                                key,
                                ..Default::default()
                            });
                        }
                        _ => {
                            state.world.entities.spawn_entity(Entity {
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
