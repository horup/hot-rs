use std::mem::transmute;


use macroquad::{
    prelude::*,
    time::get_frame_time,
    window::{screen_height, screen_width}, shapes::{draw_line, draw_rectangle_lines}, audio::{play_sound, PlaySoundParams},
};
use parry2d::{bounding_volume::BoundingVolume, na::Isometry2};
use shared::{Camera, Collision, Context, Sprites, Event, Id,  World, DrawParams};

use crate::Engine;

impl Context for Engine {
    fn draw(&mut self, camera: &Camera, world:&World, params:DrawParams) {
        self.game_camera = camera.clone();
        let bounds = self.bounds();
        let margin = 3.0;
        let bounds = Rect {
            x: bounds.x - margin,
            y: bounds.y - margin,
            w: bounds.w + margin * 2.0,
            h: bounds.h + margin * 2.0,
        };
        let mut visible_set:Vec<Id> = Vec::with_capacity(world.sprites.len());

        for (key, e) in world.sprites.iter_mut() {
            if bounds.contains(e.pos.truncate()) {
                visible_set.push(key);
            }
        }

        visible_set.sort_by(|a, b| {
            if let (Some(a), Some(b)) = (world.sprites.get(*a), world.sprites.get(*b)){
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
                if let Some(cell) = world.tiles.get(cell_x, cell_y) {
                    if !cell.hidden {
                        if let Some(tile) = cell.img_top {
                            if let Some(tex) = self.textures.get(&tile) {
                                let diffuse = cell.diffuse;
                                let c = Color { r: diffuse.r, g: diffuse.g, b: diffuse.b, a: diffuse.a  };
                                self.draw_tex(Vec2::new(cell_x as f32, cell_y as f32), tex, c);
                            }
                        }
                    }
                }
                for key in visible_set.iter() {
                    if let Some(e) = world.sprites.get(*key) {
                        if e.hidden {
                            continue;
                        }

                        if e.pos.y as i32 == cell_y {
                            if let Some(tex) = self.textures.get(&e.img) {
                                let mut diffuse = shared::WHITE;
                                if let Some(tile) = world.tiles.get(e.pos.x as i32, e.pos.y as i32) {
                                    diffuse = tile.diffuse;
                                }
                                self.draw_sprite(e.pos, tex, e.flip_x, false, Color { r: diffuse.r, g: diffuse.g, b: diffuse.b, a: diffuse.a  });
                            }
                        }
                    }
                }
            }
        }

        if params.debug_entity {
            for (_, e) in world.sprites.iter_mut() {
                let s = self.cell_size_screen() * e.radius * 2.0;
                let p = self.to_screen(e.pos.truncate());
                draw_rectangle_lines(p.x - s.x / 2.0, p.y - s.y / 2.0, s.x, s.y, 1.0, RED);
                let v = Vec2::from_angle(e.dir) * s / 2.0;
                draw_line(p.x, p.y, p.x + v.x, p.y - v.y, 1.0, BLUE);
            }
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
        let s = measure_text(&p.str, None, p.font_height as u16, 1.0);
        let x = match p.alignment_horizontal {
            shared::Alignment::Left => 0.0,
            shared::Alignment::Center => -s.width / 2.0,
            shared::Alignment::Right => -s.width,
        };
        draw_text_ex(&p.str, p.x + x, p.y, TextParams {
            font_size:p.font_height as u16,
            font_scale:1.0,
            color:Color {
                r: p.color.r,
                g: p.color.g,
                b: p.color.b,
                a: p.color.a,
            },
            ..Default::default()
        });
    }

    fn draw_img(&self, p: shared::DrawImgParams) {
        if let Some(tex) = self.textures.get(&p.img) {
            draw_texture_ex(*tex, p.x, p.y, WHITE, DrawTextureParams {
                dest_size:Some(Vec2::new(p.w, p.h)),
                ..Default::default()
            });
        }
    }

    fn draw_rect(&self, params: shared::DrawRectParams) {
        draw_rectangle(params.rect.x, params.rect.y, params.rect.w, params.rect.h, Color {
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

    fn dt(&self) -> f32 {
        get_frame_time()
    }

    fn clip_move(&self, id: Id, target: Vec3, world:&World) -> Collision {
        let mut col = Collision::default();
        if let Some(e) = world.sprites.get_mut(id) {
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
                        for (other_id, other_e) in world.sprites.iter() {
                            let ignore = e.no_clip || other_e.no_clip;
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
                            if let Some(cell) = world.tiles.get(np.x, np.y) {
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
        unsafe {
            let mut s:(Sprites, Vec<u8>) = (Sprites::default(), Vec::new());
            if let Some(game) = &*self.game.get() {
                s.1 = game.serialize();
            }
    
            bincode::serialize(&s).unwrap()
        }
    }

    fn deserialize(&mut self, bytes:&[u8]) {
        if !bytes.is_empty() {
            unsafe {
                let s:(Sprites, Vec<u8>) = bincode::deserialize(bytes).unwrap();
                let (_entities, game_bytes) = s;
                //self.entities = entities;
                if !game_bytes.is_empty() {
                    if let Some(game) = &mut *self.game.get() {
                        game.deserialize(&game_bytes);
                    }
                }
            }
        }
    }

    fn play_sound(&self, sound:u32, volume:f32) {
        if let Some(sound) = self.sounds.get(&sound) {
            play_sound(*sound, PlaySoundParams {
                volume,
                ..Default::default()
            })
        }
    }

    fn mouse_pos(&self) -> Vec2 {
        mouse_position().into()
    }

    fn mouse_button_pressed(&self, button:u8) -> bool {
        let btn: MouseButton = unsafe { transmute(button) };
        is_mouse_button_pressed(btn)
    }

    fn mouse_button_down(&self, button:u8) -> bool {
        let btn: MouseButton = unsafe { transmute(button) };
        is_mouse_button_down(btn)
    }
}
