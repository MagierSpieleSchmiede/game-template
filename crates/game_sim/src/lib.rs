//! The deterministic, headless tick simulation.
//!
//! Everything gameplay is simulated here, in `FixedUpdate`, from the intent
//! the presentation writes back — never from frame time, presentation state
//! or scattered randomness. See `docs/architecture/simulation-model.md`.

pub mod balance;
pub mod checksum;
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

/// The duration of one tick in seconds: the fixed step of the simulation,
/// never the frame delta.
pub const TICK_SECONDS: f32 = 1.0 / TICKS_PER_SECOND as f32;

/// The simulation plugin. The client adds it; the headless app adds it too.
///
/// It sets the fixed timestep to [`TICKS_PER_SECOND`] (Bevy's own default is
/// 64 Hz), so `FixedUpdate` runs at the rate the systems assume. The app
/// provides the [`scenario::Scenario`] and runs [`scenario::spawn_scenario`]
/// at startup, which also seeds the RNG.
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<balance::Balance>()
            .init_resource::<intent::IntentBuffer>()
            .init_resource::<rng::SimRng>()
            .init_resource::<ids::SimIdCounter>()
            .insert_resource(Time::<Fixed>::from_hz(TICKS_PER_SECOND as f64))
            .configure_sets(FixedUpdate, schedule::sim_sets())
            .add_systems(FixedUpdate, schedule::tick_chain());
    }
}
