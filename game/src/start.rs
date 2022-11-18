use shared::{glam::Vec3, Context, Sprite, Tiles, Color, World, Map};
use num_enum::TryFromPrimitive;
use crate::{Images, Walker, Door, MyGame, Item, sounds, State};

impl MyGame {

    pub fn start(&mut self, player_img:Images, pos:Vec3) {
        let player_entity = self.state.sprites.spawn_entity(Sprite {
            pos: pos,
            texture: player_img.into(),
            radius: 0.25,
            ..Default::default()
        });

        self.state.player = Some(player_entity);
        self.state.walkers.attach(player_entity, Walker::default());
        self.state.pause = false;
    }

    pub fn load_map(&mut self, _engine:&mut dyn Context, map:&Map) {
        let mut tiles = Tiles::from(map);
        tiles.for_each_mut(|t,_,_| {
            t.diffuse = Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            };
        });
        
        self.state = State::new(World {
            tiles,
            ..Default::default()
        });

        self.state.pause = true;

        let state = &mut self.state;
        map.grid.for_each(|cell, x, y| {
            let pos = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            if let Some(entity) = cell.entity {
                if let Ok(entity) = Images::try_from_primitive(entity) {
                    match entity {
                        Images::Viktor | Images::William => {
                           /* dbg!("Spawning Player");
                            let player_entity = state.sprites.spawn_entity(Sprite {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.25,
                                ..Default::default()
                            });

                            println!("{:?}")
    
                            state.player = Some(player_entity);
                            state.walkers.attach(player_entity, Walker::default());*/
                        }
                        Images::PokemonCard => {
                            let card = state.sprites.spawn_entity(Sprite {
                                pos, 
                                texture: entity.into(),
                                radius: 0.25,
                                no_clip: true,
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
                            let key = state.sprites.spawn_entity(Sprite {
                                pos, 
                                texture: entity.into(),
                                radius: 0.25,
                                no_clip: true,
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
                            let door = state.sprites.spawn_entity(Sprite {
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
                            state.sprites.spawn_entity(Sprite {
                                pos: Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0),
                                texture: entity.into(),
                                radius: 0.5,
                                no_clip: true,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        });
    }
}
