use shared::{Context, glam::{Vec2}, Grid, Color, Tile};
use crate::{MyGame, Images, sounds};


struct Ray {
    pub start:Vec2,
    pub end:Vec2
}
#[allow(dead_code)]
struct Visit<'a, T:Default + Clone> {
    pub tile:&'a mut T,
    pub x:f32,
    pub y:f32,
    pub d:f32
}

fn cast_ray_mut<T:Default + Clone, F:FnMut(Visit<T>)->bool>(grid:&mut Grid<T>, ray:Ray, mut f:F) {
    fn get_helper(cell_size:f32, pos:f32, dir:f32) -> (f32, f32, f32, f32) {
        let tile = (pos / cell_size).floor() + 1.0;
        let dtile;
        let dt;
        if dir > 0.0 {
            dtile = 1.0;
            dt = ((tile + 1.0) * cell_size - pos) / dir;
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
    let (mut tile_x, dtile_x, mut dt_x, ddt_x) = get_helper(1.0, ray.start.x, dir.x);
    let (mut tile_y, dtile_y, mut dt_y, ddt_y) = get_helper(1.0, ray.start.y, dir.y);

    let mut t = 0.0;
    if dir.x*dir.x + dir.y*dir.y > 0.0 {
        loop {
            if let Some(cell) = grid.get_mut(tile_x as i32, tile_y as i32) {
                if f(Visit { tile: cell, d:t, x:tile_x, y:tile_y }) {
                    break;
                }
            } else {
                break;
            }
            if dt_x < dt_y {
                tile_x += dtile_x;
                let dt = dt_x;
                t += dt;
                dt_x = dt_x + ddt_x - dt;
                dt_y -= dt;
            } else {
                tile_y += dtile_y;
                let dt = dt_y;
                t += dt;
                dt_x -= dt;
                dt_y = dt_y + ddt_y - dt;
            }
        }
    } else {
    }

}


impl MyGame {
    fn raycast_update(&mut self, _ctx: &mut dyn Context) {
        let mut los_blocked = Grid::new(self.state.tiles.size());
        for (id, door_sprite) in self.state.sprites.iter() {
            if let Some(door) = self.state.doors.get(id) {
                if door.open == false {
                    if let Some(cell) = los_blocked.get_mut(door_sprite.pos.x as i32, door_sprite.pos.y as i32) {
                        *cell = true;
                    }
                }
            }
        }

        let f = |visit:Visit<Tile>| {
            visit.tile.diffuse = Color::default();
            if let Some(blocked) = los_blocked.get(visit.x as i32, visit.y as i32) {
                if *blocked {
                    return true;
                }
            }

            visit.tile.clips
        };

        let player_id = self.state.player.unwrap_or_default();
        if let Some(player_entity) = self.state.sprites.get(player_id) {
            let pos = player_entity.pos.truncate().floor() + Vec2::new(0.5, 0.5);
            let size = self.state.tiles.size();
            for y in [0, size] {
                let y = y as f32 + 0.5;
                for x in 0..size {
                    let x = x as f32 + 0.5;
                    cast_ray_mut(&mut self.state.tiles, Ray {
                        start:pos,
                        end:Vec2::new(x, y)
                    }, f);
                }

            }

            for x in [0, size] {
                let x = x as f32 + 0.5;
                for y in 0..size {
                    let y = y as f32 + 0.5;
                    cast_ray_mut(&mut self.state.tiles, Ray {
                        start:pos,
                        end:Vec2::new(x, y)
                    }, f);
                }

            }
        }
    }

    fn proximity_update(&mut self, _ctx: &mut dyn Context) {
        let player_id = self.state.player.unwrap_or_default();
        if let Some(player_entity) = self.state.sprites.get(player_id) {
            for (other_id, other_entity) in self.state.sprites.iter().filter(|(id,_)| {id != &player_id}) {
                let v = other_entity.pos - player_entity.pos;
                let l = v.length();
                if other_entity.img == Images::ExitMarker.into() && l < 0.5 {
                    self.state.won = true;
                    self.state.pause = true;
                    break;
                } else if let Some(_critter) = self.state.critters.get(other_id) {
                    if l < 0.6 {
                        self.state.lost = true;
                        self.state.pause = true;
                        break;
                    }
                }
            }
        }
    }

    fn critter_update(&mut self, ctx:&mut dyn Context) {
        if let Some(player_id) = self.state.player {
            if let Some(player_sprite) = self.state.world.sprites.get(player_id).clone() {
                for (key, critter_sprite) in self.state.world.sprites.iter_mut() {
                    if let Some(critter) = self.state.critters.get_mut(key) {
                        let v = (player_sprite.pos - critter_sprite.pos).normalize_or_zero();
                        critter.dir = v.truncate();
                    }
                }
            }
        }
    }
    
    pub fn update(&mut self, ctx: &mut dyn Context) {
        if self.state.pause {
            return;
        }

        self.critter_update(ctx);

        let state = &mut self.state;
        let dt = ctx.dt(); 
        for (key, e) in state.world.sprites.iter_mut() {
            let speed = 3.0;
            let mut v = Vec2::default();
            if state.player == Some(key) {
                v = self.dir * speed * dt;
            }
            if let Some(critter) = state.critters.get_mut(key) {
                v = critter.dir * speed * dt;
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
            let col = ctx.clip_move(key, e.pos + v, &state);
            if let Some(other_id) = col.other_entity {
                if let Some(door) = state.doors.get_mut(other_id) {
                    let can_open = match door.key {
                        Some(tex) => state.inventory.contains_key(&tex),
                        None => true,
                    };
                    if can_open {
                        door.open_door();
                        if let Some(door) = state.sprites.get_mut(other_id) {
                            door.no_clip = true;
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
    

        if let Some(player) = state.sprites.get(state.player.unwrap_or_default()) {
            for (key, e) in state.sprites.iter_mut() {
                let v = e.pos - player.pos;
                if v.length() > 1.0 {
                    if let Some(door) = state.doors.get_mut(key) {
                        door.close_timer_sec -= dt;
                        if door.open && door.close_timer_sec <= 0.0 {
                            door.close_timer_sec = 0.0;
                            e.no_clip = false;
                            e.hidden = false;
                            door.open = false;
                            let vol = 1.0 / v.length();
                            ctx.play_sound(sounds::DOOR_CLOSE, vol);
                        }
                    }
                }
            }
        }

        let mut despawner = Vec::new();
        if let Some(player_id) = state.player {
            if let Some(player) = state.world.sprites.get(player_id) {
                for (other_id, other_entity) in state.world.sprites.iter().filter(|e| {e.0 != player_id}) {
                    let v = player.pos - other_entity.pos;
                    let r2 = player.radius + other_entity.radius;
                    if v.length() < r2 {
                        if let Some(item) = state.items.get(other_id) {
                            despawner.push(other_id);
                            ctx.play_sound(item.pickup_sound.unwrap_or(sounds::PICKUP), 1.0);
                            state.flash.flash(0.2, 0.5);

                            if other_entity.img == Images::PokemonCard.into() {
                                state.pokemon_cards.current += 1.0;
                            } else if other_entity.img == Images::GoldKey.into() {
                                state.inventory.insert(Images::GoldKey, 1.0);
                            } else if other_entity.img == Images::BlueKey.into() {
                                state.inventory.insert(Images::BlueKey, 1.0);
                            }
                        }
                    }
                }
            }
        } 

        despawner.iter().for_each(|id|{
            self.state.sprites.despawn(*id);
        });

        self.proximity_update(ctx);
        self.raycast_update(ctx);
    }
    
}

