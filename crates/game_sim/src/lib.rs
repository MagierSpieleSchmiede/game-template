//! The deterministic, headless tick simulation.
//!
//! Two layers:
//!
//! - **The core** (every module except `game`): tick counting, stable IDs,
//!   the seeded RNG, intents, scenarios, snapshots, state hashing, replays,
//!   rollback and the observation. It knows nothing about any genre.
//! - **The game** (`game`): the gameplay itself. The template ships a small
//!   example (units moving in an arena) as a *template default* (ADR-0013);
//!   a real game replaces the module and keeps the contract described in
//!   `game/mod.rs`.
//!
//! Everything gameplay is simulated in `FixedUpdate`, from the intents —
//! never from frame time, presentation state or scattered randomness. See
//! `docs/architecture/simulation-model.md`.

pub mod checksum;
pub mod game;
pub mod headless;
pub mod ids;
pub mod intent;
pub mod observation;
pub mod replay;
pub mod rng;
pub mod rollback;
pub mod scenario;
pub mod schedule;
pub mod snapshot;
pub mod tick;

use bevy::prelude::*;

/// Ticks per simulated second. All durations in the simulation are ticks
/// (ADR-0003); this constant only converts per-second balance rates. The
/// value is a template default (ADR-0014): a strategy game may want 10–20,
/// a racing game 120 or more.
pub const TICKS_PER_SECOND: i64 = 60;

/// The duration of one tick in seconds: the fixed step of the simulation,
/// never the frame delta.
pub const TICK_SECONDS: f32 = 1.0 / TICKS_PER_SECOND as f32;

/// The simulation plugin. The client adds it; the headless app adds it too.
///
/// It sets the fixed timestep to [`TICKS_PER_SECOND`] (Bevy's own default is
/// 64 Hz), adds the core resources and systems, lets the game add its own
/// (`game::build`), and starts the run from the [`scenario::Scenario`]
/// resource at `Startup`. Insert your scenario after adding the plugin.
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<scenario::Scenario>()
            .init_resource::<rng::SimRng>()
            .init_resource::<ids::SimIdCounter>()
            .init_resource::<tick::SimTick>()
            .init_resource::<intent::IntentBuffer>()
            .init_resource::<intent::TickIntents>()
            .insert_resource(Time::<Fixed>::from_hz(TICKS_PER_SECOND as f64))
            .configure_sets(FixedUpdate, schedule::sim_sets())
            .add_systems(Startup, scenario::start_run)
            .add_systems(
                FixedUpdate,
                (
                    tick::advance_tick,
                    intent::freeze_intents,
                    replay::record_tick,
                )
                    .chain()
                    .in_set(schedule::SimSet::Input),
            );
        game::build(app);
    }
}
