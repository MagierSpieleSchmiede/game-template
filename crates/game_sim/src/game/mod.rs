//! The game: the template-default example (ADR-0013) — units moving on a
//! disc-shaped arena. Replace this module with your game.
//!
//! # The contract
//!
//! The core uses exactly these items; a replacement provides all of them:
//!
//! | Item | Used by | What it is |
//! | --- | --- | --- |
//! | [`Action`] | intents | what an actor can want in a tick |
//! | [`Setup`] | scenario | what the run spawns at the start |
//! | [`build`] | `SimPlugin` | adds the game's resources and systems (in `SimSet::Update`) |
//! | [`spawn`] | `start_run` | spawns the start state; draws randomness and IDs from the core |
//! | [`GameState`], [`capture`], [`restore`] | snapshots | the complete game state as a value, and back |
//! | [`GameState::hash_into`] | state hash | feeds the state into the hasher in `SimId` order |
//! | [`GameObservation`], [`observe`] | observation | what a viewer may see, as plain data |
//!
//! Every simulation entity carries a [`SimId`]; snapshots despawn everything
//! with one before [`restore`].

mod balance;
mod systems;
mod unit;

pub use balance::Balance;
pub use unit::Unit;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::checksum::StateHasher;
use crate::ids::{SimId, SimIdCounter};
use crate::observation::Viewer;
use crate::rng::SimRng;
use crate::schedule::SimSet;

/// The kinds of things an actor can want in a tick. Keep variants stable once
/// they travel over the network (see `docs/architecture/serialization-and-ids.md`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Move along the given floor direction (x, z), magnitude up to 1;
    /// the unit keeps this velocity until its next intent.
    Move { x: f32, z: f32 },
    /// Stand still.
    Idle,
}

/// What a run spawns at the start.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Setup {
    /// How many units the arena starts with.
    pub unit_count: u32,
}

impl Default for Setup {
    fn default() -> Self {
        Self { unit_count: 2 }
    }
}

/// Adds the game's resources and systems.
pub fn build(app: &mut App) {
    app.init_resource::<Balance>()
        .configure_sets(
            FixedUpdate,
            systems::ArenaSet::chain().in_set(SimSet::Update),
        )
        .add_systems(FixedUpdate, systems::systems());
}

/// Spawns the start state: the units on the spawn circle, directions drawn
/// from the RNG, IDs assigned in spawn order.
pub fn spawn(world: &mut World, setup: &Setup) {
    let balance = world.resource::<Balance>().clone();
    for _ in 0..setup.unit_count {
        let dir = random_floor_direction(&mut world.resource_mut::<SimRng>());
        let id = world.resource_mut::<SimIdCounter>().allocate();
        world.spawn((
            id,
            Unit {
                position: Vec3::new(dir.x, 0.0, dir.y) * balance.spawn_radius,
                velocity: Vec3::ZERO,
                health: balance.unit_health,
            },
        ));
    }
}

/// The complete game state: every unit, in `SimId` order.
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    units: Vec<(SimId, Unit)>,
}

impl GameState {
    /// Feeds the state into the hasher: per unit its ID, position, velocity
    /// and health, floats by their exact bits.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        for (id, unit) in &self.units {
            hasher.write_i64(id.0);
            for v in [unit.position, unit.velocity] {
                for c in v.to_array() {
                    hasher.write_f32(c);
                }
            }
            hasher.write_i64(unit.health);
        }
    }
}

/// Captures the game state.
pub fn capture(world: &mut World) -> GameState {
    let mut units: Vec<(SimId, Unit)> = world
        .query::<(&SimId, &Unit)>()
        .iter(world)
        .map(|(id, unit)| (*id, unit.clone()))
        .collect();
    units.sort_by_key(|(id, _)| *id);
    GameState { units }
}

/// Respawns the game state into a world without simulation entities.
pub fn restore(world: &mut World, state: &GameState) {
    for (id, unit) in &state.units {
        world.spawn((*id, unit.clone()));
    }
}

/// What a viewer sees of one unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitView {
    /// The unit's ID; the presentation keys its visuals on it.
    pub id: SimId,
    /// Position on the floor (x/z, y up), in metres.
    pub position: Vec3,
    /// Remaining health in thousandths.
    pub health: i64,
}

/// What a viewer sees of the game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameObservation {
    /// The radius of the arena floor, in metres.
    pub arena_radius: f32,
    /// The visible units, in `SimId` order.
    pub units: Vec<UnitView>,
}

/// Observes the game. The arena has no teams and no fog of war, so every
/// viewer sees every unit; a game with hidden information filters here, by
/// `viewer`, and nowhere else.
pub fn observe(world: &mut World, viewer: Viewer) -> GameObservation {
    let _ = viewer;
    let mut units: Vec<UnitView> = world
        .query::<(&SimId, &Unit)>()
        .iter(world)
        .map(|(id, unit)| UnitView {
            id: *id,
            position: unit.position,
            health: unit.health,
        })
        .collect();
    units.sort_by_key(|u| u.id);
    GameObservation {
        arena_radius: world.resource::<Balance>().arena_radius,
        units,
    }
}

/// A uniformly distributed unit vector, drawn without `sin`/`cos`: those come
/// from the platform's maths library and may differ in the last bit between
/// platforms. Rejection sampling in the unit square uses only `+ - * /` and
/// `sqrt`, which IEEE 754 defines exactly, so every platform gets the same bits.
fn random_floor_direction(rng: &mut SimRng) -> Vec2 {
    loop {
        let v = Vec2::new(rng.unit_f32() * 2.0 - 1.0, rng.unit_f32() * 2.0 - 1.0);
        let len_sq = v.length_squared();
        if len_sq > 1e-4 && len_sq <= 1.0 {
            return v / len_sq.sqrt();
        }
    }
}
