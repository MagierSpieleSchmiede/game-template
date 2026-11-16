use std::time::Duration;

use bevy::prelude::*;
use bevy::time::Fixed;

use game_sim::game::{Action, Balance, Setup, Unit};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::scenario::Scenario;
use game_sim::tick::SimTick;

fn push(app: &mut App, unit: i64, action: Action) {
    app.world_mut().resource_mut::<IntentBuffer>().push(Intent {
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
