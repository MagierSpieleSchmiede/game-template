//! The run (template default, ADR-0016): what a new run starts from and who
//! plays it at this client. The menu starts every run from [`RunSetup`]; a
//! lobby or the server fills it in a real game.

use bevy::prelude::*;

use game_sim::control::PlayerId;
use game_sim::observation::{TeamId, Viewer};
use game_sim::scenario::{Scenario, start_run};

/// The setup of the next run started from the menu.
#[derive(Debug, Clone, PartialEq, Resource)]
pub struct RunSetup {
    /// The start state: seed and the game's setup.
    pub scenario: Scenario,
    /// The player at this client: its intents carry this ID, and it steers
    /// the actor the observation says it controls.
    pub player: PlayerId,
    /// Whose view this client shows.
    pub viewer: Viewer,
}

impl Default for RunSetup {
    /// The template's single-player run: the default scenario, played by the
    /// first player, seen by the first team.
    fn default() -> Self {
        Self {
            scenario: Scenario::default(),
            player: PlayerId(0),
            viewer: Viewer::Team(TeamId(0)),
        }
    }
}

/// Starts a new run from [`RunSetup`]: whatever ran before is replaced by the
/// scenario's start state, and the presentation observes it afresh.
pub fn start_new_run(world: &mut World) {
    let scenario = world.resource::<RunSetup>().scenario.clone();
    world.insert_resource(scenario);
    start_run(world);
    crate::presentation::observe_restored(world);
}

#[cfg(test)]
mod tests {
    use bevy::app::TaskPoolPlugin;
    use bevy::state::app::StatesPlugin;
    use bevy::time::{TimePlugin, TimeUpdateStrategy};

    use game_sim::SimPlugin;
    use game_sim::checksum::state_hash;
    use game_sim::game::Setup;
    use game_sim::headless::headless_app;
    use game_sim::ids::SimId;
    use game_sim::tick::SimTick;

    use super::*;
    use crate::input::{ActionState, InputAction};
    use crate::localisation::Localisation;
    use crate::presentation::Frames;
    use crate::screens::Screen;

    /// The client's screen flow and run lifecycle without window or devices:
    /// time advances one tick per update, as in the headless app.
    fn client(setup: RunSetup) -> App {
        let mut app = App::new();
        app.add_plugins((
            TaskPoolPlugin::default(),
            TimePlugin,
            StatesPlugin,
            SimPlugin,
        ))
        .insert_resource(TimeUpdateStrategy::FixedTimesteps(1))
        .insert_resource(setup)
        .insert_resource(Localisation::english())
        .init_resource::<ActionState>()
        .insert_resource(Frames {
            hash_wanted: true,
            ..default()
        })
        .add_plugins(crate::screens::plugin)
        .add_systems(FixedPreUpdate, crate::presentation::write_intent)
        .add_systems(FixedPostUpdate, crate::presentation::observe_tick);
        app.update();
        app
    }

    /// One frame with `held` held and nothing else.
    fn frame(app: &mut App, held: &[InputAction]) {
        let mut actions = ActionState::default();
        for action in held {
            actions.press(*action);
        }
        app.insert_resource(actions);
        app.update();
    }

    fn screen(app: &App) -> Screen {
        *app.world().resource::<State<Screen>>().get()
    }

    fn position(app: &App, id: i64) -> Vec3 {
        let frames = app.world().resource::<Frames>();
        let current = frames.current.as_ref().expect("observed");
        current
            .game
            .units
            .iter()
            .find(|u| u.id.0 == id)
            .unwrap()
            .position
    }

    #[test]
    fn menu_run_menu_run_starts_each_run_from_its_scenario() {
        let first = Scenario {
            seed: 3,
            setup: Setup { unit_count: 3 },
        };
        let mut app = client(RunSetup {
            scenario: first,
            player: PlayerId(1),
            viewer: Viewer::Spectator,
        });
        assert_eq!(screen(&app), Screen::Menu);

        frame(&mut app, &[InputAction::Confirm]);
        frame(&mut app, &[]);
        assert_eq!(screen(&app), Screen::Playing);
        let start = (position(&app, 1), position(&app, 2));
        for _ in 0..20 {
            frame(&mut app, &[InputAction::MoveRight]);
        }
        assert!(*app.world().resource::<SimTick>() > SimTick(10));
        assert_eq!(position(&app, 1), start.0, "unit 1 is player 0's");
        assert!(
            position(&app, 2).x > start.1.x,
            "the local player 1 steers its unit 2"
        );

        frame(&mut app, &[InputAction::Pause]);
        frame(&mut app, &[InputAction::QuitToMenu]);
        frame(&mut app, &[]);
        assert_eq!(screen(&app), Screen::Menu);

        let second = Scenario {
            seed: 8,
            setup: Setup { unit_count: 4 },
        };
        app.world_mut().resource_mut::<RunSetup>().scenario = second.clone();
        frame(&mut app, &[InputAction::Confirm]);
        assert_eq!(*app.world().resource::<SimTick>(), SimTick(0));
        let fresh = state_hash(headless_app(second).world_mut());
        assert_eq!(state_hash(app.world_mut()), fresh);
        let frames = app.world().resource::<Frames>();
        assert_eq!(
            frames.hash,
            Some(fresh),
            "the presentation shows the new run"
        );
        let ids: Vec<SimId> = frames
            .current
            .as_ref()
            .unwrap()
            .game
            .units
            .iter()
            .map(|u| u.id)
            .collect();
        assert_eq!(ids, (1..=4).map(SimId).collect::<Vec<_>>());
    }
}
