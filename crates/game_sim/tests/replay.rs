use game_sim::control::PlayerId;
use game_sim::game::{Action, Setup};
use game_sim::headless::{headless_app, step_ticks};
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::replay::{finish_recording, play, start_recording};
use game_sim::scenario::Scenario;

/// A recorded run plays back to the same final state.
#[test]
fn replay_reproduces_the_run() {
    let mut app = headless_app(Scenario {
        seed: 5,
        setup: Setup { unit_count: 3 },
    });
    start_recording(&mut app);
    for round in 0..4 {
        app.world_mut().resource_mut::<IntentBuffer>().push(Intent {
            player: PlayerId((1 + round % 3) as u32 - 1),
            actor: SimId(1 + round % 3),
            action: Action::Move { x: 0.7, z: -0.7 },
        });
        step_ticks(&mut app, 25);
    }
    let replay = finish_recording(&mut app);
    assert_eq!(replay.ticks.len(), 100);

    let (_, hash) = play(&replay);
    assert_eq!(hash, replay.final_hash);
}
