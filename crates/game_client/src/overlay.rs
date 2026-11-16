//! The debug overlay (template default, ADR-0016): tick, state hash and frame
//! rate in a corner, toggled by the `toggle-debug` action. The state hash is
//! what two machines compare to find a desync.

use bevy::prelude::*;

use crate::input::{ActionState, InputAction};
use crate::localisation::Localisation;
use crate::presentation::Frames;

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

fn toggle(actions: Res<ActionState>, mut overlay: Single<&mut Visibility, With<DebugOverlay>>) {
    if actions.just_pressed(InputAction::ToggleDebug) {
        overlay.toggle_visible_hidden();
    }
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
    let fps = 1.0 / time.delta_secs().max(1e-6);
    text.0 = texts.text(
        "debug-overlay",
        &[
            ("tick", &tick.to_string()),
            ("hash", &format!("{:016x}", frames.hash)),
            ("fps", &format!("{fps:.0}")),
        ],
    );
}
