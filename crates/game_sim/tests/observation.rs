use game_sim::control::PlayerId;
use game_sim::game::Setup;
use game_sim::headless::{headless_app, step_ticks};
use game_sim::observation::{TeamId, Viewer, observe};
use game_sim::scenario::Scenario;
use game_sim::tick::SimTick;

/// The observation reports the tick and every unit, in `SimId` order, with
/// the player controlling it.
#[test]
fn observation_lists_units_in_id_order() {
    let mut app = headless_app(Scenario {
        seed: 4,
        setup: Setup { unit_count: 5 },
    });
    step_ticks(&mut app, 3);
    let seen = observe(app.world_mut(), Viewer::Team(TeamId(0)));
    assert_eq!(seen.tick, SimTick(3));
    let ids: Vec<i64> = seen.game.units.iter().map(|u| u.id.0).collect();
    assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    let controllers: Vec<Option<PlayerId>> = seen.game.units.iter().map(|u| u.controller).collect();
    assert_eq!(
        controllers,
        (0..5).map(|p| Some(PlayerId(p))).collect::<Vec<_>>()
    );
}
