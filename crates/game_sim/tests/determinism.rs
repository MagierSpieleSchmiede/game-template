use bevy::prelude::*;

use game_sim::checksum::state_hash;
use game_sim::game::{Action, Setup, Unit};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::scenario::Scenario;

/// The scripted run of the determinism tests: four units, moves in several
/// directions (normalisation), long enough to hit the arena edge (clamping).
/// A new game replaces the script with one that exercises its own systems.
pub fn scripted_run(seed: u64) -> App {
    let mut app = headless_app(Scenario {
        seed,
        setup: Setup { unit_count: 4 },
    });
    let script = [
        (1, Action::Move { x: 1.0, z: 0.0 }),
        (2, Action::Move { x: -0.6, z: 0.8 }),
        (3, Action::Move { x: 3.0, z: -4.0 }),
        (4, Action::Idle),
    ];
    for round in 0..5 {
        let mut buffer = app.world_mut().resource_mut::<IntentBuffer>();
        for (unit, action) in script {
            let action = match (round % 2, action) {
                (1, Action::Move { x, z }) => Action::Move { x: -z, z: x },
                (_, a) => a,
            };
            buffer.push(Intent {
                actor: SimId(unit),
                action,
            });
        }
        step_ticks(&mut app, 120);
    }
    app
}

/// Units in `SimId` order (ADR-0008), never in query order.
fn units(app: &mut App) -> Vec<(SimId, Unit)> {
    let mut units: Vec<_> = app
        .world_mut()
        .query::<(&SimId, &Unit)>()
        .iter(app.world())
        .map(|(id, u)| (*id, u.clone()))
        .collect();
    units.sort_by_key(|(id, _)| *id);
    units
}

/// The determinism contract (determinism.md): same scenario, same inputs, same
/// seed → same end state. Two runs must agree on every unit, and the script
/// must actually have changed the state, or the comparison proves nothing.
#[test]
fn same_scenario_same_end_state() {
    let fresh = units(&mut headless_app(Scenario {
        seed: 42,
        setup: Setup { unit_count: 4 },
    }));
    let (mut a, mut b) = (scripted_run(42), scripted_run(42));
    let (ua, ub) = (units(&mut a), units(&mut b));

    assert_eq!(ua.len(), 4);
    assert_eq!(ua, ub);
    for ((id, end), (_, start)) in ua.iter().zip(&fresh) {
        if id.0 != 4 {
            assert_ne!(end.position, start.position, "unit {id:?} never moved");
        }
    }
    assert_eq!(state_hash(a.world_mut()), state_hash(b.world_mut()));
}

/// The end state of the scripted run, as a committed value. CI runs this on
/// Linux, Windows and macOS, so it checks the contract across platforms, not
/// only within one process. If a deliberate change to the simulation changes
/// the value, update it in the same commit and say why.
#[test]
fn scripted_run_matches_committed_hash() {
    let hash = state_hash(scripted_run(42).world_mut());
    assert_eq!(
        hash, 0x04ea_21ca_6db7_cc82,
        "state hash changed: {hash:#018x}"
    );
}

/// Different seeds must produce different runs.
#[test]
fn different_seed_diverges() {
    let (mut a, mut b) = (scripted_run(1), scripted_run(2));
    assert_ne!(state_hash(a.world_mut()), state_hash(b.world_mut()));
}
