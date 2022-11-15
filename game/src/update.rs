use shared::{Context, glam::{Vec2, IVec2}, IgnoreColissions, Command, Grid};
use crate::{MyGame, Images, sounds};


struct Ray {
    pub start:Vec2,
    pub end:Vec2
}

struct Visit<'a, T:Default + Clone> {
    tile:&'a mut T
}

fn cast_ray_mut<T:Default + Clone, F:FnMut(Visit<T>)->bool>(grid:&mut Grid<T>, ray:Ray, mut f:F) {
    fn get_helper(cell_size:f32, pos:f32, dir:f32) -> (f32, f32, f32, f32) {
        let tile = (pos / cell_size).floor() + 1.0;
        let dtile;
        let dt;
        if dir > 0.0 {
            dtile = 1.0;
            dt = ((tile + 0.0) * cell_size - pos) / dir;
        } else {
            dtile = -1.0;
            dt = ((tile - 1.0) * cell_size - pos) / dir;
        }

        (tile, dtile, dt, dtile * cell_size / dir)
    }
    let dir = (ray.end - ray.start).normalize_or_zero();
    if dir.length() == 0.0 {
        return;
    }
    let (mut tile_x, dtile_x, mut dt_x, ddt_x) = get_helper(1.0, ray.start.x, dir.y);
    let (mut tile_y, dtile_y, mut dt_y, ddt_y) = get_helper(1.0, ray.start.y, dir.y);

    let mut t = 0.0;
    if dir.x*dir.x + dir.y*dir.y > 0.0 {
        loop {//tile_x >= 0.0 && tile_x <= grid.size() as f32 && tile_y > 0.0 && tile_y <= grid.size() as f32 {
            if let Some(cell) = grid.get_mut(tile_x as i32, tile_y as i32) {
                f(Visit { tile: cell });
            } else {
                break;
            }
            if dt_x < dt_y {
                tile_x = tile_x + dtile_x;
                let dt = dt_x;
                t = t + dt;
                dt_x = dt_x + ddt_x - dt;
                dt_y = dt_y - dt;
            } else {
                tile_y = tile_y + dtile_y;
                let dt = dt_y;
                t = t + dt;
                dt_x = dt_x - dt;
                dt_y = dt_y + ddt_y - dt;
            }
        }
    } else {
        println!("true");
    }

}


impl MyGame {

    fn raycast_update(&mut self, ctx: &mut dyn Context) {
        let player_id = self.state.player.unwrap_or_default();
        if let Some(player_entity) = ctx.entities().get(player_id) {
            let pos = player_entity.pos.truncate();

            cast_ray_mut(ctx.tiles_mut(), Ray {
                start:pos,
                end:Vec2::new(0.0, 0.0)
            }, |visit|{
                visit.tile.hidden = false;
                return visit.tile.clips;
            });
          /*   if let Some(tile) = ctx.tiles_mut().get_mut(posi.x, posi.y) {
                tile.hidden = false;
            }*/
        }
    }

    fn proximity_update(&mut self, ctx: &mut dyn Context) {
        let player_id = self.state.player.unwrap_or_default();
        if let Some(player_entity) = ctx.entities().get(player_id) {
            for (other_id, other_entity) in ctx.entities().iter().filter(|(id,_)| {id != &player_id}) {
                let v = other_entity.pos - player_entity.pos;
                let l = v.length();
                if other_entity.texture == Images::ExitMarker.into() {
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

                            if other_entity.texture == Images::PokemonCard.into() {
                                state.pokemon_cards.current += 1.0;
                            } else if other_entity.texture == Images::GoldKey.into() {
                                state.inventory.insert(Images::GoldKey, 1.0);
                            } else if other_entity.texture == Images::BlueKey.into() {
                                state.inventory.insert(Images::BlueKey, 1.0);
                            }
                        }
                    }
                });
            }
        } 

        self.proximity_update(ctx);
        self.raycast_update(ctx);
    }
    
}

