use bevy::prelude::*;

use game_sim::checksum::state_hash;
use game_sim::control::PlayerId;
use game_sim::game::{Action, Setup};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::rng::SimRng;
use game_sim::scenario::Scenario;
use game_sim::schedule::SimSet;
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
            player: PlayerId(actor as u32 - 1),
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
/// intents again — both futures must be identical. A forgotten component or
/// run-mutable resource keeps its later value across the rewind, so the
/// second future differs once that state feeds back into hashed state (see
/// `resources_left_out_of_the_snapshot_turn_the_check_red`).
#[test]
fn snapshots_are_complete() {
    let mut app = running_app();
    assert!(futures_agree(&mut app, |_, _| {}));
}

/// Runs the same 40 ticks twice from a snapshot taken now; `restore_extra`
/// runs right after the restore, for state a game captures alongside its
/// `GameState`. True when both futures end in the same state hash.
fn futures_agree(app: &mut App, restore_extra: impl Fn(&mut World, &Countdown)) -> bool {
    let snapshot = Snapshot::capture(app.world_mut());
    let saved = app
        .world()
        .get_resource::<Countdown>()
        .cloned()
        .unwrap_or_default();

    let future = |app: &mut App| {
        push_moves(app);
        step_ticks(app, 40);
        state_hash(app.world_mut())
    };
    let first = future(app);
    snapshot.restore(app.world_mut());
    restore_extra(app.world_mut(), &saved);
    let second = future(app);
    first == second
}

/// A run-mutable resource of the kind a real game has (a timer, a score, an
/// objective): it counts ticks, and from tick 50 on the game draws from the
/// RNG every tick. The example game has none, so the test adds it.
#[derive(Resource, Clone, Default, Debug)]
struct Countdown(u64);

fn tick_countdown(mut countdown: ResMut<Countdown>, mut rng: ResMut<SimRng>) {
    countdown.0 += 1;
    if countdown.0 >= 50 {
        rng.next_u64();
    }
}

fn running_app_with_countdown() -> App {
    let mut app = headless_app(Scenario {
        seed: 9,
        setup: Setup { unit_count: 3 },
    });
    app.init_resource::<Countdown>()
        .add_systems(FixedUpdate, tick_countdown.in_set(SimSet::Update));
    push_moves(&mut app);
    step_ticks(&mut app, 30);
    app
}

/// A run-mutable resource the snapshot leaves out turns the completeness
/// check red; restored with the snapshot, it stays green. The contract in
/// `game/mod.rs` therefore puts every such resource into `GameState`.
#[test]
fn resources_left_out_of_the_snapshot_turn_the_check_red() {
    let mut forgotten = running_app_with_countdown();
    assert!(!futures_agree(&mut forgotten, |_, _| {}));

    let mut captured = running_app_with_countdown();
    assert!(futures_agree(&mut captured, |world, saved| {
        world.insert_resource(saved.clone());
    }));
}
