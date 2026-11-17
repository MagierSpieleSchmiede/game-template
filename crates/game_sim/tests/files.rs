use bevy::prelude::*;

use game_sim::checksum::state_hash;
use game_sim::control::PlayerId;
use game_sim::file::FileError;
use game_sim::game::{Action, Setup};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::replay::{Replay, finish_recording, play, start_recording};
use game_sim::save::SaveGame;
use game_sim::scenario::Scenario;

fn scenario() -> Scenario {
    Scenario {
        seed: 21,
        setup: Setup { unit_count: 3 },
    }
}

fn push_moves(app: &mut App, round: u32) {
    let mut buffer = app.world_mut().resource_mut::<IntentBuffer>();
    for unit in 1..=3i64 {
        let turn = (round as f32 + unit as f32) * 0.7;
        buffer.push(Intent {
            player: PlayerId(unit as u32 - 1),
            actor: SimId(unit),
            action: Action::Move {
                x: 1.0 - turn.fract(),
                z: turn.fract() - 0.5,
            },
        });
    }
}

fn run_on(app: &mut App, rounds: std::ops::Range<u32>) {
    for round in rounds {
        push_moves(app, round);
        step_ticks(app, 20);
    }
}

/// A save restored from its bytes continues the run exactly as if it had
/// never stopped.
#[test]
fn a_loaded_save_continues_the_run_exactly() {
    let mut original = headless_app(scenario());
    run_on(&mut original, 0..3);
    let bytes = SaveGame::capture(original.world_mut()).to_bytes();

    let mut loaded = SaveGame::from_bytes(&bytes).unwrap().headless_app();
    assert_eq!(
        state_hash(loaded.world_mut()),
        state_hash(original.world_mut())
    );

    run_on(&mut original, 3..8);
    run_on(&mut loaded, 3..8);
    assert_eq!(
        state_hash(loaded.world_mut()),
        state_hash(original.world_mut())
    );
}

/// A save whose state was changed after saving is refused.
#[test]
fn an_edited_save_is_refused() {
    let mut app = headless_app(scenario());
    run_on(&mut app, 0..2);
    let mut save = SaveGame::capture(app.world_mut());
    save.state_hash ^= 1;
    assert!(matches!(
        SaveGame::from_bytes(&save.to_bytes()),
        Err(FileError::HashMismatch { .. })
    ));
}

/// Damage anywhere in a save is caught: as a broken payload or as a state that
/// no longer matches its hash. Never as a silently different run.
#[test]
fn every_flipped_byte_of_a_save_is_caught() {
    let mut app = headless_app(scenario());
    run_on(&mut app, 0..2);
    let bytes = SaveGame::capture(app.world_mut()).to_bytes();
    for i in 0..bytes.len() {
        let mut damaged = bytes.clone();
        damaged[i] ^= 0x20;
        if let Ok(save) = SaveGame::from_bytes(&damaged) {
            // Only bytes that do not affect the state may survive (e.g. a
            // changed seed in the scenario, which is not replayed on load).
            assert_eq!(
                save.snapshot,
                SaveGame::from_bytes(&bytes).unwrap().snapshot
            );
        }
    }
}

/// A replay survives the trip through bytes and still plays back to its hash.
#[test]
fn a_replay_file_plays_back() {
    let mut app = headless_app(scenario());
    start_recording(&mut app);
    run_on(&mut app, 0..5);
    let replay = finish_recording(&mut app);

    let loaded = Replay::from_bytes(&replay.to_bytes()).unwrap();
    assert_eq!(loaded, replay);
    assert_eq!(play(&loaded).1, replay.final_hash);
}

/// A save is not a replay, and the other way round.
#[test]
fn saves_and_replays_are_not_interchangeable() {
    let mut app = headless_app(scenario());
    start_recording(&mut app);
    step_ticks(&mut app, 3);
    let save = SaveGame::capture(app.world_mut()).to_bytes();
    let replay = finish_recording(&mut app).to_bytes();
    assert!(matches!(
        Replay::from_bytes(&save),
        Err(FileError::WrongKind { .. })
    ));
    assert!(matches!(
        SaveGame::from_bytes(&replay),
        Err(FileError::WrongKind { .. })
    ));
}
