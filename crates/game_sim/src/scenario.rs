//! Scenarios: the complete start state of a simulation run, as data.

use bevy::prelude::*;

use crate::{balance::Balance, ids::SimId, rng::SimRng, unit::Unit};

/// A scenario: everything a run needs to be reproducible — the seed for the
/// RNG and the units to spawn. Same scenario, same inputs → same end state.
#[derive(Debug, Clone, Resource)]
pub struct Scenario {
    /// Seed for [`SimRng`].
    pub seed: u64,
    /// How many units the arena starts with.
    pub unit_count: u32,
}

impl Default for Scenario {
    fn default() -> Self {
        Self {
            seed: 0,
            unit_count: 2,
        }
    }
}

/// Spawns the scenario's units deterministically: each on the arena circle,
/// angles drawn from the seeded RNG, IDs assigned in spawn order.
pub fn spawn_scenario(
    mut commands: Commands,
    scenario: Res<Scenario>,
    balance: Res<Balance>,
    mut rng: ResMut<SimRng>,
) {
    let mut counter = 0;
    for _ in 0..scenario.unit_count {
        let angle = rng.next_u64() as f32 / u64::MAX as f32 * std::f32::consts::TAU;
        let r = balance.arena_radius * 0.5;
        commands.spawn(Unit {
            id: SimId::next(&mut counter),
            position: Vec3::new(r * angle.cos(), 0.0, r * angle.sin()),
            velocity: Vec3::ZERO,
            health: 1000,
        });
    }
}
