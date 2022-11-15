use shared::{Context, glam::{Vec2}, IgnoreColissions, Command};
use crate::{MyGame, Textures, sounds};

impl MyGame {
    fn proximity_update(&mut self, ctx: &mut dyn Context) {
        let player_id = self.state.player.unwrap_or_default();
        if let Some(player_entity) = ctx.entities().get(player_id) {
            for (other_id, other_entity) in ctx.entities().iter().filter(|(id,_)| {id != &player_id}) {
                let v = other_entity.pos - player_entity.pos;
                let l = v.length();
                if other_entity.texture == Textures::ExitMarker.into() {
                    if l < 0.5 {
                        // TODO end game
                    }
                }
            }
        }
    }
    pub fn update(&mut self, ctx: &mut dyn Context) {
        let state = &mut self.state;
        let dt = ctx.dt(); 
        for (key, e) in ctx.entities().iter_mut() {
            let speed = 3.0;
            let mut v = Vec2::default();
            if state.player == Some(key) {
                v = self.dir * speed * dt;
            }
    
            if v.x > 0.0 {
                e.flip_x = false;
            } else if v.x < 0.0 {
                e.flip_x = true;
            }
    
            if v.length() > 0.0 {
                let d = v.angle_between(Vec2::new(1.0, 0.0));
                e.dir = d;
            }
    
            if let Some(walker) = state.walkers.get_mut(key) {
                let walking = v.length() > 0.0;
                if walking {
                    walker.walker += v.length() * 2.0;
                    if walker.walker > 1.0 {
                        walker.walker = 0.0;
                    }
    
                    e.pos.z = if walker.walker > 0.5 { 0.1 } else { 0.0 };
                } else {
                    e.pos.z = 0.0;
                }
            }
    
            let v = v.extend(0.0);
            let col = ctx.clip_move(key, e.pos + v);
            if let Some(other_id) = col.other_entity {
                if let Some(door) = state.doors.get_mut(other_id) {
                    let can_open = match door.key {
                        Some(tex) => state.inventory.contains_key(&tex),
                        None => true,
                    };
                    if can_open {
                        door.open_door();
                        if let Some(door) = ctx.entities().get_mut(other_id) {
                            door.ignore_collisions = IgnoreColissions::WithEntities;
                            door.hidden = true;
                            ctx.play_sound(sounds::DOOR_OPEN, 1.0);
                        }
                    }
                }
            }
            
            if state.player == Some(key) {
                state.camera.zoom = 12.0;
                state.camera.pos = e.pos.truncate(); 
            }
        }
    

        if let Some(player) = ctx.entities().get(state.player.unwrap_or_default()) {
            for (key, e) in ctx.entities().iter_mut() {
                let v = e.pos - player.pos;
                if v.length() > 1.0 {
                    if let Some(door) = state.doors.get_mut(key) {
                        door.close_timer_sec -= dt;
                        if door.open && door.close_timer_sec <= 0.0 {
                            door.close_timer_sec = 0.0;
                            e.ignore_collisions = IgnoreColissions::None;
                            e.hidden = false;
                            door.open = false;
                            let vol = 1.0 / v.length();
                            ctx.play_sound(sounds::DOOR_CLOSE, vol);
                        }
                    }
                }
            }
        }

        if let Some(player_id) = state.player {
            if let Some(player) = ctx.entities().get(player_id) {
                ctx.entities().iter().filter(|e| {e.0 != player_id}).for_each(|(other_id, other_entity)| {
                    let v = player.pos - other_entity.pos;
                    let r2 = player.radius + other_entity.radius;
                    if v.length() < r2 {
                        if let Some(item) = state.items.get(other_id) {
                            ctx.push_command(Command::DespawnEntity{
                                id:other_id
                            });
                            ctx.play_sound(item.pickup_sound.unwrap_or(sounds::PICKUP), 1.0);
                            state.flash(0.2, 0.5);

                            if other_entity.texture == Textures::PokemonCard.into() {
                                state.pokemon_cards.current += 1.0;
                            } else if other_entity.texture == Textures::GoldKey.into() {
                                state.inventory.insert(Textures::GoldKey, 1.0);
                            } else if other_entity.texture == Textures::BlueKey.into() {
                                state.inventory.insert(Textures::BlueKey, 1.0);
                            }
                        }
                    }
                });
            }
        } 

        self.proximity_update(ctx);
    }
    
}

