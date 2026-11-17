//! Replays: a run is fully described by its scenario and the intents that
//! arrived in each tick. They are recorded as they arrived, before the checks,
//! so playback runs the checks again: a change to validation or control shows
//! up as a different result, like any other change. Recording them gives replays, spectating after the fact, bug reports
//! that reproduce, and regression tests (play an old replay, compare hashes).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::checksum::state_hash;
use crate::file::{self, FileError, FileKind};
use crate::headless::headless_app;
use crate::intent::{IntentBuffer, TickIntents};
use crate::scenario::Scenario;

/// A recorded run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Replay {
    /// The start state.
    pub scenario: Scenario,
    /// The intents that arrived for every tick, tick 1 first, in arrival
    /// order and unchecked.
    pub ticks: Vec<TickIntents>,
    /// The state hash after the last tick: playback must reach it.
    pub final_hash: u64,
}

impl Replay {
    /// The replay as file bytes (ADR-0017).
    pub fn to_bytes(&self) -> Vec<u8> {
        file::encode(FileKind::Replay, self)
    }

    /// Reads a replay, refusing other files and replays recorded with
    /// another simulation version.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FileError> {
        file::decode(FileKind::Replay, bytes)
    }
}

/// The recording in progress. Insert it to start recording; without it,
/// nothing is recorded.
#[derive(Debug, Clone, Default, Resource)]
pub struct ReplayLog(pub Vec<TickIntents>);

/// Appends the intents that arrived for this tick to the [`ReplayLog`], if
/// one is recording. Runs before they are checked and frozen.
pub fn record_tick(log: Option<ResMut<ReplayLog>>, buffer: Res<IntentBuffer>) {
    if let Some(mut log) = log {
        log.0.push(TickIntents(buffer.intents.clone()));
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
