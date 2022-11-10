use std::mem::transmute;

use macroquad::{
    prelude::{get_last_key_pressed, is_key_down, is_key_pressed, KeyCode, Vec2, Vec3},
    time::get_frame_time,
    window::{screen_height, screen_width},
};
use parry2d::{bounding_volume::BoundingVolume, na::Isometry2};
use shared::{Camera, Collision, Context, Entities, Event, Id, IgnoreColissions};

use crate::Engine;

impl Context for Engine {
    fn map(&self) -> &shared::Map {
        &self.map
    }

    fn draw_world(&mut self, camera: &Camera) {
        self.game_camera = camera.clone();
        self.draw_game_mode();
    }

    fn screen_size(&self) -> shared::glam::Vec2 {
        Vec2::new(screen_width(), screen_height())
    }

    fn texture_size(&self, texture: u32) -> shared::glam::Vec2 {
        if let Some(tex) = self.textures.get(&texture) {
            return Vec2::new(tex.width(), tex.height());
        }

        Vec2::new(0.0, 0.0)
    }

    fn draw_string(&self, _params: shared::DrawStringParams) {
        todo!()
    }

    fn draw_texture(&self, _params: shared::DrawTextureParams) {
        todo!()
    }

    fn draw_rect(&self, _params: shared::DrawRectParams) {
        todo!()
    }

    fn push_command(&mut self, command: shared::Command) {
        self.commands.push(command);
    }

    fn events(&mut self) -> Vec<Event> {
        let mut events = Vec::with_capacity(self.events.len());
        std::mem::swap(&mut self.events, &mut events);
        return events;
    }

    fn is_key_pressed(&self, key_code: u8) -> bool {
        let key_code: KeyCode = unsafe { transmute(key_code) };
        return is_key_pressed(key_code);
    }

    fn is_key_down(&self, key_code: u8) -> bool {
        let key_code: KeyCode = unsafe { transmute(key_code) };
        return is_key_down(key_code);
    }

    fn last_key_pressed(&self) -> Option<u8> {
        if let Some(key_code) = get_last_key_pressed() {
            let k: u8 = unsafe { transmute(key_code) };
            return Some(k);
        }
        None
    }

    fn entities(&self) -> &Entities {
        &self.entities
    }

    fn dt(&self) -> f32 {
        get_frame_time()
    }

    fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }

    fn clip_move(&self, id: Id, target: Vec3) -> Collision {
        let mut col = Collision::default();
        if let Some(e) = self.entities.get_mut(id) {
            let v = target - e.pos;
            if v.length() > 0.0 {
                let mut left = v.length();
                let d = v.normalize();

                // FIXME: max step should be configurable at some point
                let max_step = 1.0 / 16.0;
                const DIMS: [Vec2; 2] = [Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)];
                while left > 0.0 {
                    let mut step = left;
                    if step > max_step {
                        step = max_step;
                    }
                    let v = d * step;
                    left -= step;

                    for dim in DIMS {
                        let pos_org = e.pos;
                        let v = v.truncate() * dim;
                        if v.length() == 0.0 {
                            continue;
                        }

                        let mut pos_new = pos_org + v.extend(0.0);

                        // collision handling between entities
                        for (other_id, other_e) in self.entities.iter() {
                            let ignore = e.ignore_collisions == IgnoreColissions::WithEntities
                                || other_e.ignore_collisions == IgnoreColissions::WithEntities;
                            if other_id != id && !ignore {
                                let d = e.pos - other_e.pos;
                                let r2 = e.radius + other_e.radius;
                                if d.length() < r2 {
                                    let r = r2 - d.length();
                                    let v = d.normalize() * r;
                                    pos_new += v;

                                    // FIXME: last collision is saved, even though multiple might exist
                                    col.other_entity = Some(other_id);
                                }
                            }
                        }

                        // collision between grid
                        let v = pos_new - pos_org;
                        let v = v.truncate() * dim;
                        let d = v.normalize();
                        let rev_dim = Vec2::new(dim.y, dim.x);
                        for i in [-1, 0, 1] {
                            let i = i as f32;
                            let cp = Vec2::new(i, i) * rev_dim + d + pos_org.truncate();
                            let np = cp.as_ivec2();
                            if let Some(cell) = self.map.grid.get(np.x, np.y) {
                                if cell.blocks {
                                    let s1 =
                                        parry2d::shape::Cuboid::new([e.radius, e.radius].into());
                                    let s1_pos = Isometry2::translation(pos_new.x, pos_new.y);
                                    let aabb1 = s1.aabb(&s1_pos);
                                    let s2 = parry2d::shape::Cuboid::new([0.5, 0.5].into());
                                    let s2_pos = Isometry2::translation(
                                        np.x as f32 + 0.5,
                                        np.y as f32 + 0.5,
                                    );
                                    let aabb2 = s2.aabb(&s2_pos);

                                    if aabb1.intersects(&aabb2) {
                                        pos_new = pos_org;

                                        // FIXME: might override the entity collision which occured before
                                        col.tile = Some(np);
                                        break;
                                    }
                                }
                            }
                        }

                        e.pos = pos_new;
                    }
                }
            }
        }
        
        col
    }
}
