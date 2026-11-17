//! The debug overlay (template default, ADR-0016): tick, state hash and frame
//! rate in a corner, toggled by the `toggle-debug` action. The state hash is
//! what two machines compare to find a desync. It reads the whole world, so
//! it is computed only while the overlay is visible.

use bevy::prelude::*;

use crate::input::{ActionState, InputAction};
use crate::localisation::Localisation;
use crate::presentation::{Frames, refresh_hash};

#[derive(Debug, Component)]
struct DebugOverlay;

/// Adds the overlay, hidden at start.
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn)
        .add_systems(Update, (toggle, update).chain());
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        DebugOverlay,
        Text::default(),
        TextFont::from_font_size(14.0),
        Node {
            position_type: PositionType::Absolute,
            top: px(8),
            left: px(8),
            ..default()
        },
        Visibility::Hidden,
    ));
}

/// Shows or hides the overlay, and asks for the state hash only while it is
/// visible. A shown overlay gets the hash at once, even while paused.
fn toggle(world: &mut World) {
    if !world
        .resource::<ActionState>()
        .just_pressed(InputAction::ToggleDebug)
    {
        return;
    }
    let mut overlays = world.query_filtered::<&mut Visibility, With<DebugOverlay>>();
    let Ok(mut visibility) = overlays.single_mut(world) else {
        return;
    };
    visibility.toggle_visible_hidden();
    let visible = *visibility != Visibility::Hidden;
    world.resource_mut::<Frames>().hash_wanted = visible;
    refresh_hash(world);
}

fn update(
    frames: Res<Frames>,
    texts: Res<Localisation>,
    time: Res<Time<Real>>,
    mut overlay: Single<(&mut Text, &Visibility), With<DebugOverlay>>,
) {
    let (text, visibility) = &mut *overlay;
    if **visibility == Visibility::Hidden {
        return;
    }
    let tick = frames.current.as_ref().map_or(0, |o| o.tick.0);
    let hash = frames
        .hash
        .map_or_else(String::new, |h| format!("{h:016x}"));
    let fps = 1.0 / time.delta_secs().max(1e-6);
    text.0 = texts.text(
        "debug-overlay",
        &[
            ("tick", &tick.to_string()),
            ("hash", &hash),
            ("fps", &format!("{fps:.0}")),
        ],
    );
}

#[cfg(test)]
mod tests {
    use bevy::app::TaskPoolPlugin;
    use bevy::time::{TimePlugin, TimeUpdateStrategy};

    use game_sim::SimPlugin;
    use game_sim::checksum::state_hash;
    use game_sim::scenario::Scenario;

    use super::*;
    use crate::run::RunSetup;

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((TaskPoolPlugin::default(), TimePlugin, SimPlugin))
            .insert_resource(TimeUpdateStrategy::FixedTimesteps(1))
            .insert_resource(Scenario::default())
            .init_resource::<RunSetup>()
            .insert_resource(Localisation::english())
            .init_resource::<ActionState>()
            .init_resource::<Frames>()
            .add_systems(FixedPostUpdate, crate::presentation::observe_tick)
            .add_plugins(plugin);
        app.update();
        app
    }

    fn frame(app: &mut App, toggle: bool) {
        let mut actions = ActionState::default();
        if toggle {
            actions.press(InputAction::ToggleDebug);
        }
        app.insert_resource(actions);
        app.update();
    }

    fn hash(app: &App) -> Option<u64> {
        app.world().resource::<Frames>().hash
    }

    #[test]
    fn the_state_hash_is_computed_only_while_the_overlay_is_visible() {
        let mut app = app();
        for _ in 0..5 {
            frame(&mut app, false);
        }
        assert_eq!(hash(&app), None, "hidden overlay, no hash");

        frame(&mut app, true);
        let shown = state_hash(app.world_mut());
        assert_eq!(hash(&app), Some(shown), "shown at once, before any tick");
        frame(&mut app, false);
        assert_eq!(hash(&app), Some(state_hash(app.world_mut())));
        assert_ne!(hash(&app), Some(shown), "follows each tick");

        frame(&mut app, true);
        frame(&mut app, false);
        assert_eq!(hash(&app), None, "hidden again, no hash");
    }
}
