use std::time::Duration;

use bevy::prelude::*;
use bevy::time::Fixed;

use game_sim::control::PlayerId;
use game_sim::game::{Action, Balance, Setup, Unit};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer, RejectedIntents, Rejection};
use game_sim::scenario::Scenario;
use game_sim::tick::SimTick;

fn push(app: &mut App, unit: i64, action: Action) {
    app.world_mut().resource_mut::<IntentBuffer>().push(Intent {
        player: PlayerId(unit as u32 - 1),
        actor: SimId(unit),
        action,
    });
}

fn unit(app: &mut App, id: i64) -> Unit {
    app.world_mut()
        .query::<(&SimId, &Unit)>()
        .iter(app.world())
        .find(|(sid, _)| sid.0 == id)
        .map(|(_, u)| u.clone())
        .expect("unit exists")
}

/// A `Move` intent must reach its unit and move it at the balance speed.
#[test]
fn move_intent_moves_the_unit() {
    let mut app = headless_app(Scenario::default());
    let start = unit(&mut app, 1).position;
    push(&mut app, 1, Action::Move { x: 1.0, z: 0.0 });

    step_ticks(&mut app, game_sim::TICKS_PER_SECOND as u32);

    let moved = unit(&mut app, 1).position - start;
    let speed = app.world().resource::<Balance>().move_speed;
    assert!(
        (moved.x - speed).abs() < 1e-3 && moved.z.abs() < 1e-6,
        "one second of Move{{x: 1}} should move by {speed} m along x, moved {moved:?}"
    );
}

/// The arena is a disc on the x/z floor: units stay inside it and on the floor.
#[test]
fn units_stay_inside_the_arena_on_the_floor() {
    let mut app = headless_app(Scenario::default());
    push(&mut app, 1, Action::Move { x: 0.0, z: 1.0 });
    step_ticks(&mut app, 60 * game_sim::TICKS_PER_SECOND as u32);

    let radius = app.world().resource::<Balance>().arena_radius;
    let p = unit(&mut app, 1).position;
    assert_eq!(p.y, 0.0, "units stay on the floor");
    assert!(
        Vec2::new(p.x, p.z).length() <= radius + 1e-4,
        "unit left the arena: {p:?}"
    );
}

/// The fixed timestep must match TICKS_PER_SECOND, or the client simulates at
/// another rate than the one the systems assume.
#[test]
fn fixed_timestep_matches_ticks_per_second() {
    let app = headless_app(Scenario::default());
    let expected = Duration::from_secs_f64(1.0 / game_sim::TICKS_PER_SECOND as f64);
    assert_eq!(app.world().resource::<Time<Fixed>>().timestep(), expected);
}

/// IDs handed out after the scenario spawn continue the sequence.
#[test]
fn spawn_counter_continues_after_the_scenario() {
    let mut app = headless_app(Scenario {
        seed: 3,
        setup: Setup { unit_count: 4 },
    });
    let next = app
        .world_mut()
        .resource_mut::<game_sim::ids::SimIdCounter>()
        .allocate();
    assert_eq!(next, SimId(5));
}

/// The tick counter starts at 0 and counts every tick.
#[test]
fn tick_counter_counts_ticks() {
    let mut app = headless_app(Scenario::default());
    assert_eq!(*app.world().resource::<SimTick>(), SimTick(0));
    step_ticks(&mut app, 7);
    assert_eq!(*app.world().resource::<SimTick>(), SimTick(7));
}

/// Intents are applied in actor order, whatever order they arrived in: the
/// last intent per actor wins, independent of arrival between actors.
#[test]
fn intents_are_frozen_in_actor_order() {
    let mut app = headless_app(Scenario::default());
    push(&mut app, 2, Action::Idle);
    push(&mut app, 1, Action::Idle);
    app.update();
    let frozen = app.world().resource::<game_sim::intent::TickIntents>();
    let actors: Vec<i64> = frozen.0.iter().map(|i| i.actor.0).collect();
    assert_eq!(actors, vec![1, 2]);
}

fn push_as(app: &mut App, player: u32, unit: i64, action: Action) {
    app.world_mut().resource_mut::<IntentBuffer>().push(Intent {
        player: PlayerId(player),
        actor: SimId(unit),
        action,
    });
}

/// An action with NaN or infinity is rejected before any system sees it; the
/// unit keeps its state instead of turning into NaN for the rest of the run.
#[test]
fn malformed_actions_are_rejected() {
    let mut app = headless_app(Scenario::default());
    let before = unit(&mut app, 1);
    push(
        &mut app,
        1,
        Action::Move {
            x: f32::NAN,
            z: 0.0,
        },
    );
    push(
        &mut app,
        2,
        Action::Move {
            x: f32::INFINITY,
            z: 1.0,
        },
    );
    app.update();

    assert_eq!(unit(&mut app, 1).velocity, before.velocity);
    assert!(unit(&mut app, 2).position.is_finite());
    let rejected = &app.world().resource::<RejectedIntents>().0;
    assert_eq!(rejected.len(), 2);
    assert!(
        rejected
            .iter()
            .all(|(_, why)| *why == Rejection::InvalidAction)
    );
}

/// A player can only steer the actors it controls.
#[test]
fn intents_for_actors_of_other_players_are_rejected() {
    let mut app = headless_app(Scenario::default());
    let start = unit(&mut app, 2).position;
    // Player 0 controls unit 1, not unit 2.
    push_as(&mut app, 0, 2, Action::Move { x: 1.0, z: 0.0 });
    step_ticks(&mut app, 10);

    assert_eq!(unit(&mut app, 2).position, start);
    let rejected = &app.world().resource::<RejectedIntents>().0;
    // The rejection list holds only the latest tick; the intent came at tick 1.
    assert!(rejected.is_empty());

    push_as(&mut app, 0, 2, Action::Move { x: 1.0, z: 0.0 });
    app.update();
    let rejected = &app.world().resource::<RejectedIntents>().0;
    assert_eq!(rejected.len(), 1);
    assert_eq!(rejected[0].1, Rejection::NotInControl);
}

/// Starting a run again in a world that already ran one must give the same
/// state as a fresh app: the old run leaves no units, IDs, control, tick or
/// RNG behind, and the new run goes on exactly like a fresh one.
#[test]
fn start_run_again_starts_from_the_scenario() {
    let first = Scenario {
        seed: 1,
        setup: Setup { unit_count: 5 },
    };
    let second = Scenario {
        seed: 9,
        setup: Setup { unit_count: 3 },
    };
    let mut app = headless_app(first);
    push(&mut app, 2, Action::Move { x: 1.0, z: 0.0 });
    step_ticks(&mut app, 40);
    push(&mut app, 1, Action::Move { x: 0.0, z: 1.0 });

    app.world_mut().insert_resource(second.clone());
    game_sim::scenario::start_run(app.world_mut());

    let mut fresh = headless_app(second);
    assert_eq!(*app.world().resource::<SimTick>(), SimTick(0));
    assert_eq!(
        game_sim::checksum::state_hash(app.world_mut()),
        game_sim::checksum::state_hash(fresh.world_mut())
    );
    step_ticks(&mut app, 30);
    step_ticks(&mut fresh, 30);
    assert_eq!(
        game_sim::checksum::state_hash(app.world_mut()),
        game_sim::checksum::state_hash(fresh.world_mut()),
        "no intent of the old run leaks into the new one"
    );
}
