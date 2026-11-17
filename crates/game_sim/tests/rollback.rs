use game_sim::checksum::state_hash;
use game_sim::control::PlayerId;
use game_sim::game::{Action, Setup};
use game_sim::headless::headless_app;
use game_sim::ids::SimId;
use game_sim::intent::Intent;
use game_sim::rollback::{RollbackSession, TooLate};
use game_sim::scenario::Scenario;
use game_sim::tick::SimTick;

fn scenario() -> Scenario {
    Scenario {
        seed: 11,
        setup: Setup { unit_count: 3 },
    }
}

fn late_intent() -> Intent {
    Intent {
        player: PlayerId(1),
        actor: SimId(2),
        action: Action::Move { x: 0.0, z: 1.0 },
    }
}

fn on_time(tick: u64) -> Vec<Intent> {
    let mut intents = vec![Intent {
        player: PlayerId(0),
        actor: SimId(1),
        action: Action::Move { x: 1.0, z: 0.0 },
    }];
    if tick == 5 {
        intents.push(late_intent());
    }
    intents
}

/// An intent that arrives late is rolled back into its tick: the result equals
/// the run in which it arrived on time.
#[test]
fn late_intent_matches_the_on_time_run() {
    let mut reference = RollbackSession::new(headless_app(scenario()), 16);
    for tick in 1..=12 {
        reference.advance(on_time(tick));
    }

    let mut late = RollbackSession::new(headless_app(scenario()), 16);
    for tick in 1..=12 {
        let mut intents = on_time(tick);
        intents.retain(|i| i.actor != SimId(2));
        late.advance(intents);
    }
    let before = state_hash(late.app_mut().world_mut());
    let resimulated = late.insert_late(SimTick(5), late_intent()).unwrap();

    assert_eq!(resimulated, 8);
    assert_eq!(late.tick(), SimTick(12));
    let after = state_hash(late.app_mut().world_mut());
    assert_ne!(after, before, "the late intent changed nothing");
    assert_eq!(after, state_hash(reference.app_mut().world_mut()));
}

/// Intents older than the rollback window are refused.
#[test]
fn too_late_outside_the_window() {
    let mut session = RollbackSession::new(headless_app(scenario()), 4);
    for tick in 1..=10 {
        session.advance(on_time(tick));
    }
    assert_eq!(
        session.insert_late(SimTick(3), late_intent()),
        Err(TooLate {
            oldest: Some(SimTick(7))
        })
    );
}
