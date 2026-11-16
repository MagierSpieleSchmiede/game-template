//! The screen flow (template default, ADR-0016): menu → playing ⇄ paused.
//! The simulation runs on virtual time, so pausing virtual time pauses it.

use bevy::prelude::*;

use crate::input::{ActionState, InputAction};
use crate::localisation::Localisation;

/// Which screen the client shows.
#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Screen {
    /// The start screen; the simulation waits.
    #[default]
    Menu,
    /// The simulation runs.
    Playing,
    /// The simulation is paused.
    Paused,
}

/// Adds the screen flow.
pub fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .add_systems(OnEnter(Screen::Menu), (pause_time, spawn_menu))
        .add_systems(OnEnter(Screen::Playing), resume_time)
        .add_systems(OnEnter(Screen::Paused), (pause_time, spawn_paused))
        .add_systems(Update, navigate);
}

fn pause_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

fn navigate(
    screen: Res<State<Screen>>,
    actions: Res<ActionState>,
    mut next: ResMut<NextState<Screen>>,
) {
    let target = match *screen.get() {
        Screen::Menu if actions.just_pressed(InputAction::Confirm) => Screen::Playing,
        Screen::Playing if actions.just_pressed(InputAction::Pause) => Screen::Paused,
        Screen::Paused
            if actions.just_pressed(InputAction::Pause)
                || actions.just_pressed(InputAction::Confirm) =>
        {
            Screen::Playing
        }
        _ => return,
    };
    next.set(target);
}

fn centred_column() -> Node {
    Node {
        width: percent(100),
        height: percent(100),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        row_gap: px(16),
        ..default()
    }
}

fn spawn_menu(mut commands: Commands, texts: Res<Localisation>) {
    commands.spawn((
        DespawnOnExit(Screen::Menu),
        centred_column(),
        children![
            (
                Text::new(texts.text("menu-title", &[])),
                TextFont::from_font_size(48.0)
            ),
            Text::new(texts.text("menu-start", &[])),
        ],
    ));
}

fn spawn_paused(mut commands: Commands, texts: Res<Localisation>) {
    commands.spawn((
        DespawnOnExit(Screen::Paused),
        centred_column(),
        children![Text::new(texts.text("paused", &[]))],
    ));
}
