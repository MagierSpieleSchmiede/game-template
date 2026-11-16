use bevy::prelude::*;

use game_sim::headless::{headless_app, step_ticks};
use game_sim::scenario::Scenario;
use game_sim::unit::Unit;

/// The determinism contract (determinism.md): same scenario, same inputs, same
/// seed → same end state, byte for byte. Two runs of the same scenario must
/// agree on every unit's position and health.
#[test]
fn same_scenario_same_end_state() {
    let scenario = Scenario {
        seed: 42,
        unit_count: 4,
    };
    let mut a = headless_app(scenario.clone());
    let mut b = headless_app(scenario);

    // Identical scripted intents in both runs.
    for app in [&mut a, &mut b] {
        let mut buffer = app
            .world_mut()
            .resource_mut::<game_sim::intent::IntentBuffer>();
        buffer.push(game_sim::intent::Intent {
            unit: game_sim::ids::SimId(1),
            action: game_sim::intent::Action::Move { x: 1.0, z: 0.0 },
        });
    }

    step_ticks(&mut a, 120);
    step_ticks(&mut b, 120);

    let units = |app: &mut App| -> Vec<Unit> {
        app.world_mut()
            .query::<&Unit>()
            .iter(app.world())
            .cloned()
            .collect()
    };
    let (ua, ub) = (units(&mut a), units(&mut b));
    assert_eq!(ua.len(), ub.len());
    for (x, y) in ua.iter().zip(ub.iter()) {
        assert_eq!(x.id, y.id);
        assert_eq!(x.position, y.position);
        assert_eq!(x.health, y.health);
    }
}

/// Different seeds must produce different runs.
#[test]
fn different_seed_diverges() {
    let mut a = headless_app(Scenario {
        seed: 1,
        unit_count: 4,
    });
    let mut b = headless_app(Scenario {
        seed: 2,
        unit_count: 4,
    });
    step_ticks(&mut a, 10);
    step_ticks(&mut b, 10);
    let pa: Vec<_> = a
        .world_mut()
        .query::<&Unit>()
        .iter(a.world())
        .map(|u| u.position)
        .collect();
    let pb: Vec<_> = b
        .world_mut()
        .query::<&Unit>()
        .iter(b.world())
        .map(|u| u.position)
        .collect();
    assert_ne!(pa, pb);
}
