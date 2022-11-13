use std::mem::transmute;


use macroquad::{
    prelude::*,
    time::get_frame_time,
    window::{screen_height, screen_width}, shapes::{draw_line, draw_rectangle_lines}, audio::play_sound_once,
};
use parry2d::{bounding_volume::BoundingVolume, na::Isometry2};
use shared::{Camera, Collision, Context, Entities, Event, Id, IgnoreColissions};

use crate::Engine;

impl Context for Engine {
    fn map(&self) -> &shared::Map {
        &self.map
    }

    fn draw(&mut self, camera: &Camera) {
        self.game_camera = camera.clone();
        let bounds = self.bounds();
        let margin = 3.0;
        let bounds = Rect {
            x: bounds.x - margin,
            y: bounds.y - margin,
            w: bounds.w + margin * 2.0,
            h: bounds.h + margin * 2.0,
        };
        let mut visible_set:Vec<Id> = Vec::with_capacity(self.entities.len());

        for (key, e) in self.entities.iter_mut() {
            if bounds.contains(e.pos.truncate()) {
                visible_set.push(key);
            }
        }

        visible_set.sort_by(|a, b| {
            if let (Some(a), Some(b)) = (self.entities.get(*a), self.entities.get(*b)){
                if a.pos.y < b.pos.y {
                    return std::cmp::Ordering::Less;
                } else if a.pos.y > b.pos.y {
                    return std::cmp::Ordering::Greater;
                }
            }

            std::cmp::Ordering::Equal
        });

        for cell_y in bounds.top() as i32 .. bounds.bottom() as i32 {
            for cell_x in bounds.left() as i32 .. bounds.right() as i32 {
                if let Some(cell) = self.world.get(cell_x, cell_y) {

                    if !cell.hidden {
                        if let Some(tile) = cell.img {
                            if let Some(tex) = self.textures.get(&tile) {
                                self.draw_tex(Vec2::new(cell_x as f32, cell_y as f32), tex);
                            }
                        }
                    }
                }
                for key in visible_set.iter() {
                    if let Some(e) = self.entities.get(*key) {
                        if e.hidden {
                            continue;
                        }

                        if e.pos.y as i32 == cell_y {
                            if let Some(tex) = self.textures.get(&e.texture) {
                                self.draw_sprite(e.pos, tex, e.flip_x, false);
                            }
                        }
                    }
                }
            }
        }

        for (_, e) in self.entities.iter_mut() {
            let s = self.cell_size_screen() * e.radius * 2.0;
            let p = self.to_screen(e.pos.truncate());
            draw_rectangle_lines(p.x - s.x / 2.0, p.y - s.y / 2.0, s.x, s.y, 1.0, RED);
            let v = Vec2::from_angle(e.dir) * s / 2.0;
            draw_line(p.x, p.y, p.x + v.x, p.y - v.y, 1.0, BLUE);
        }
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

    fn draw_string(&self, p: shared::DrawStringParams) {
        draw_text_ex(&p.str, p.x, p.y, TextParams {
            ..Default::default()
        });
    }

    fn draw_texture(&self, p: shared::DrawTextureParams) {
        if let Some(tex) = self.textures.get(&p.texture) {
            draw_texture_ex(*tex, p.x, p.y, WHITE, DrawTextureParams {
                dest_size:Some(Vec2::new(p.w, p.h)),
                ..Default::default()
            });
        }
    }

    fn draw_rect(&self, params: shared::DrawRectParams) {
        draw_rectangle(params.x, params.y, params.w, params.h, Color {
            r:params.color.r,
            g:params.color.g,
            b:params.color.b,
            a:params.color.a
        })
    }

    fn push_command(&self, command: shared::Command) {
        self.commands.borrow_mut().push(command);
    }

    fn events(&mut self) -> Vec<Event> {
        let mut events = Vec::with_capacity(self.events.len());
        std::mem::swap(&mut self.events, &mut events);
        events
    }

    fn is_key_pressed(&self, key_code: u8) -> bool {
        let key_code: KeyCode = unsafe { transmute(key_code) };
        is_key_pressed(key_code)
    }

    fn is_key_down(&self, key_code: u8) -> bool {
        let key_code: KeyCode = unsafe { transmute(key_code) };
        is_key_down(key_code)
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
                            if let Some(cell) = self.world.get(np.x, np.y) {
                                if cell.clips {
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

    fn serialize(&self) -> Vec<u8> {
        let mut s:(Entities, Vec<u8>) = (self.entities.clone(), Vec::new());
        if let Some(game) = &self.game {
            s.1 = game.serialize();
        }

        bincode::serialize(&s).unwrap()
    }

    fn deserialize(&mut self, bytes:&[u8]) {
        if !bytes.is_empty() {
            let s:(Entities, Vec<u8>) = bincode::deserialize(bytes).unwrap();
            let (entities, game_bytes) = s;
            self.entities = entities;
            if !game_bytes.is_empty() {
                if let Some(game) = &mut self.game {
                    game.deserialize(&game_bytes);
                }
            }
        }
    }

    fn play_sound(&self, sound:u32) {
        if let Some(sound) = self.sounds.get(&sound) {
            play_sound_once(*sound);
        }
    }

    fn world(&self) -> &shared::World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut shared::World {
        &mut self.world
    }
}
