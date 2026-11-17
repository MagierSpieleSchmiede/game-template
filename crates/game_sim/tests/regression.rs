//! The committed regression replay: a recorded run, played back by every CI
//! platform. It fails when the simulation's behaviour changes, when the file
//! format changes, or when a platform computes differently.
//!
//! When a change is *meant* to alter behaviour: bump `game::SIM_VERSION`, then
//! re-record and commit the new file:
//!
//! ```sh
//! cargo test -p game_sim --test regression -- --ignored record_regression_replay
//! ```
//!
//! A new game replaces [`script`] with a run that exercises its own systems.

use bevy::prelude::*;

use game_sim::control::PlayerId;
use game_sim::file::FileError;
use game_sim::game::{Action, Setup};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::replay::{Replay, finish_recording, play, start_recording};
use game_sim::scenario::Scenario;

const FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/regression.replay");
const RERECORD: &str =
    "cargo test -p game_sim --test regression -- --ignored record_regression_replay";

/// Five players, moves in all directions, runs to the arena edge, and intents
/// that must be rejected (malformed, or for another player's unit).
fn script(app: &mut App) {
    for round in 0..12u32 {
        let mut buffer = app.world_mut().resource_mut::<IntentBuffer>();
        for unit in 1..=5i64 {
            let k = (round as i64 * 3 + unit * 7) % 11;
            let action = match k {
                0 => Action::Idle,
                1 => Action::Move {
                    x: f32::NAN,
                    z: 0.0,
                },
                _ => Action::Move {
                    x: (k as f32 - 5.0) / 3.0,
                    z: (5.0 - (k * 2 % 11) as f32) / 4.0,
                },
            };
            let player = if k == 2 {
                unit as u32 % 5
            } else {
                unit as u32 - 1
            };
            buffer.push(Intent {
                player: PlayerId(player),
                actor: SimId(unit),
                action,
            });
        }
        step_ticks(app, 50);
    }
}

fn record() -> Replay {
    let mut app = headless_app(Scenario {
        seed: 2026,
        setup: Setup { unit_count: 5 },
    });
    start_recording(&mut app);
    script(&mut app);
    finish_recording(&mut app)
}

/// The committed replay still plays back to its recorded final state.
#[test]
fn regression_replay_reproduces() {
    let bytes =
        std::fs::read(FILE).unwrap_or_else(|e| panic!("{FILE}: {e}; record it: {RERECORD}"));
    let replay = match Replay::from_bytes(&bytes) {
        Ok(replay) => replay,
        Err(e @ FileError::SimVersion { .. }) => {
            panic!("{e}: re-record the regression replay for this version: {RERECORD}")
        }
        Err(e) => panic!("cannot read {FILE}: {e}"),
    };
    let (_, hash) = play(&replay);
    assert_eq!(
        hash, replay.final_hash,
        "the simulation no longer reproduces the committed replay. If the change was \
         meant to alter behaviour, bump game::SIM_VERSION and run: {RERECORD}"
    );
}

/// The script still records exactly the committed file, so the two cannot
/// drift apart unnoticed. Compared as bytes: the script sends NaN on purpose,
/// and NaN never compares equal to itself.
#[test]
fn script_matches_the_committed_replay() {
    let committed = std::fs::read(FILE).expect("regression replay exists");
    if Replay::from_bytes(&committed).is_ok() {
        assert!(
            record().to_bytes() == committed,
            "the script changed; re-record: {RERECORD}"
        );
    }
}

/// The replay holds intents as they arrived, rejected ones included, so
/// playback runs the checks again.
#[test]
fn regression_replay_includes_rejected_intents() {
    let replay = record();
    let nan = replay
        .ticks
        .iter()
        .flat_map(|t| &t.0)
        .filter(|i| matches!(i.action, Action::Move { x, .. } if x.is_nan()))
        .count();
    let foreign = replay
        .ticks
        .iter()
        .flat_map(|t| &t.0)
        .filter(|i| i.player.0 != i.actor.0 as u32 - 1)
        .count();
    assert!(nan > 0 && foreign > 0, "nan {nan}, foreign {foreign}");
}

/// Re-records the regression replay. Run only on purpose (see the module docs).
#[test]
#[ignore = "writes tests/data/regression.replay; run on purpose to re-record"]
fn record_regression_replay() {
    let replay = record();
    std::fs::write(FILE, replay.to_bytes()).expect("write the regression replay");
    println!(
        "recorded {} ticks, final hash {:#018x}, {} bytes",
        replay.ticks.len(),
        replay.final_hash,
        replay.to_bytes().len()
    );
}
