//! Replays: a run is fully described by its scenario and the intents of each
//! tick. Recording them gives replays, spectating after the fact, bug reports
//! that reproduce, and regression tests (play an old replay, compare hashes).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::checksum::state_hash;
use crate::headless::headless_app;
use crate::intent::{IntentBuffer, TickIntents};
use crate::scenario::Scenario;

/// A recorded run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Replay {
    /// The start state.
    pub scenario: Scenario,
    /// The frozen intents of every tick, tick 1 first.
    pub ticks: Vec<TickIntents>,
    /// The state hash after the last tick: playback must reach it.
    pub final_hash: u64,
}

/// The recording in progress. Insert it to start recording; without it,
/// nothing is recorded.
#[derive(Debug, Clone, Default, Resource)]
pub struct ReplayLog(pub Vec<TickIntents>);

/// Appends this tick's intents to the [`ReplayLog`], if one is recording.
pub fn record_tick(log: Option<ResMut<ReplayLog>>, intents: Res<TickIntents>) {
    if let Some(mut log) = log {
        log.0.push(intents.clone());
    }
}

/// Starts recording in `app` from the next tick on. Start before the first
/// tick to record a whole run.
pub fn start_recording(app: &mut App) {
    app.insert_resource(ReplayLog::default());
}

/// Stops recording and returns the replay, ending in the current state.
pub fn finish_recording(app: &mut App) -> Replay {
    let ticks = app
        .world_mut()
        .remove_resource::<ReplayLog>()
        .expect("recording was started")
        .0;
    Replay {
        scenario: app.world().resource::<Scenario>().clone(),
        ticks,
        final_hash: state_hash(app.world_mut()),
    }
}

/// Plays a replay in a fresh headless app and returns the app and its final
/// state hash; equal to `replay.final_hash` if the simulation is unchanged.
pub fn play(replay: &Replay) -> (App, u64) {
    let mut app = headless_app(replay.scenario.clone());
    for tick in &replay.ticks {
        app.world_mut().resource_mut::<IntentBuffer>().intents = tick.0.clone();
        app.update();
    }
    let hash = state_hash(app.world_mut());
    (app, hash)
}
