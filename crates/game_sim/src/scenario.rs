//! Scenarios: the complete start state of a simulation run, as data.

use bevy::prelude::*;

use crate::{balance::Balance, ids::SimIdCounter, rng::SimRng, unit::Unit};

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

/// Starts the run from the scenario: seeds [`SimRng`] from the scenario's seed
/// (the one place that happens, for the client and the headless app alike),
/// then spawns the units on the spawn circle, directions drawn from the RNG,
/// IDs assigned in spawn order.
pub fn spawn_scenario(
    mut commands: Commands,
    scenario: Res<Scenario>,
    balance: Res<Balance>,
    mut rng: ResMut<SimRng>,
    mut ids: ResMut<SimIdCounter>,
) {
    *rng = SimRng::seeded(scenario.seed);
    for _ in 0..scenario.unit_count {
        let dir = random_floor_direction(&mut rng);
        commands.spawn((
            ids.allocate(),
            Unit {
                position: Vec3::new(dir.x, 0.0, dir.y) * balance.spawn_radius,
                velocity: Vec3::ZERO,
                health: balance.unit_health,
            },
        ));
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
