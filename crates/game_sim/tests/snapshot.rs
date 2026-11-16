use bevy::prelude::*;

use game_sim::checksum::state_hash;
use game_sim::game::{Action, Setup};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::scenario::Scenario;
use game_sim::snapshot::Snapshot;

fn running_app() -> App {
    let mut app = headless_app(Scenario {
        seed: 9,
        setup: Setup { unit_count: 3 },
    });
    push_moves(&mut app);
    step_ticks(&mut app, 30);
    app
}

fn push_moves(app: &mut App) {
    let mut buffer = app.world_mut().resource_mut::<IntentBuffer>();
    for (actor, x) in [(1, 1.0), (2, -1.0), (3, 0.5)] {
        buffer.push(Intent {
            actor: SimId(actor),
            action: Action::Move { x, z: 0.3 },
        });
    }
}

/// Restoring a snapshot reproduces the captured state exactly.
#[test]
fn restore_reproduces_the_captured_state() {
    let mut app = running_app();
    let snapshot = Snapshot::capture(app.world_mut());
    let hash = state_hash(app.world_mut());

    step_ticks(&mut app, 50);
    assert_ne!(state_hash(app.world_mut()), hash);

    snapshot.restore(app.world_mut());
    assert_eq!(state_hash(app.world_mut()), hash);
}

/// The completeness check: run on from a snapshot, rewind, run the same
/// intents again — both futures must be identical. State the game forgets to
/// capture (a resource, a component) makes the second future differ.
#[test]
fn snapshots_are_complete() {
    let mut app = running_app();
    let snapshot = Snapshot::capture(app.world_mut());

    let future = |app: &mut App| {
        push_moves(app);
        step_ticks(app, 40);
        state_hash(app.world_mut())
    };
    let first = future(&mut app);
    snapshot.restore(app.world_mut());
    let second = future(&mut app);
    assert_eq!(first, second);
}
