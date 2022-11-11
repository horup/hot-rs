use shared::{Context, glam::{Vec2, Vec3}, IgnoreColissions, Command};
use crate::MyGame;

impl MyGame {
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
                    door.open_door();
                }
            }
            
            if state.player == Some(key) {
                state.camera.zoom = 12.0;
                state.camera.pos = e.pos.truncate(); 
            }
        }
    
        for (key, e) in ctx.entities().iter_mut() {
            if let Some(door) = state.doors.get_mut(key) {
                if door.open {
                    e.ignore_collisions = IgnoreColissions::WithEntities;
                    e.hidden = true;
                } else {
                    e.hidden = false;
                }
    
                door.close_timer_sec -= dt;
                if door.close_timer_sec <= 0.0 {
                    door.close_timer_sec = 0.0;
                    e.ignore_collisions = IgnoreColissions::None;
                    door.open = false;
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

                            state.flash(0.2, 0.5);
                        }
                    }
                });
            }
        } 
    }
    
}

