mod entity;

use std::num::NonZeroU32;

pub use entity::*;

mod command;
pub use command::*;
mod map;

pub use map::*;
mod edit;
pub use edit::*;
mod grid;
pub use grid::*;
mod camera;
pub use camera::*;

pub use glam;
use glam::Vec2;

use serde::{Serialize, Deserialize};
pub use slotmap;
use slotmap::{new_key_type, SlotMap, Key, KeyData};

use rapier2d::prelude::*;
pub use rapier2d;


#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub texture: u32,
}

#[derive(Default)]
pub struct PlayerInput {
    pub dir: Vec2,
    pub action: bool,
    pub mouse_pos_screen: Vec2,
    pub mouse_pos_world: Vec2,
    pub mouse_left_down: bool,
    pub mouse_right_down: bool,
    pub mouse_left_pressed: bool,
    pub mouse_right_pressed: bool,
}

new_key_type! { pub struct EntityKey; }

#[derive(Default)]
pub struct Context {
    pub edit_camera: Camera,
    pub game_camera: Camera,
    pub over_ui: bool,
    pub edit_mode: bool,
    pub map: Map,
    pub entities: SlotMap<EntityKey, Entity>,
    pub tilemap: Grid<Tile>,
    pub commands: Vec<Command>,
    pub input: PlayerInput,
    pub debug: bool,
    pub edit: Edit,
    pub dt: f32,
    pub physics:Physics
}

pub struct Physics {
    pub integration_parameters:IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager : IslandManager,
    pub broad_phase : BroadPhase,
    pub narrow_phase : NarrowPhase,
    pub impulse_joint_set : ImpulseJointSet,
    pub multibody_joint_set : MultibodyJointSet,
    pub ccd_solver : CCDSolver
}

impl Default for Physics {
    fn default() -> Self {
        Self { integration_parameters: Default::default(), physics_pipeline: Default::default(), island_manager: Default::default(), broad_phase: Default::default(), narrow_phase: Default::default(), impulse_joint_set: Default::default(), multibody_joint_set: Default::default(), ccd_solver: Default::default() }
    }
}

impl Context {
    pub fn define_texture(&mut self, handle: impl Into<u32>, src: &str) {
        self.commands.push(Command::DefineTexture {
            handle: handle.into(),
            path: src.into(),
        })
    }
}
