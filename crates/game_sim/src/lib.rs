//! The deterministic, headless tick simulation.
//!
//! Everything gameplay is simulated here, in `FixedUpdate`, from the intent
//! the presentation writes back — never from frame time, presentation state
//! or scattered randomness. See `docs/architecture/simulation-model.md`.

pub mod balance;
pub mod headless;
pub mod ids;
pub mod intent;
pub mod rng;
pub mod scenario;
pub mod schedule;
pub mod unit;

use bevy::prelude::*;

/// Ticks per simulated second. All durations in the simulation are ticks
/// (ADR-0003); this constant only converts per-second balance rates.
pub const TICKS_PER_SECOND: i64 = 60;

/// The simulation plugin. The client adds it; the headless app adds it too.
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<balance::Balance>()
            .init_resource::<intent::IntentBuffer>()
            .init_resource::<rng::SimRng>()
            .add_systems(FixedUpdate, schedule::tick_chain());
    }
}
