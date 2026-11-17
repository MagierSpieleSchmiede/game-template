//! Input through actions (template default, ADR-0016): gameplay code asks
//! "is `Confirm` pressed?", "where does the move axis point?", never about a
//! key. Keyboard and gamepad feed the same actions; the default bindings live
//! in `data/input/bindings.txt`.

use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy::reflect::enums::{DynamicEnum, DynamicVariant};

/// Everything the player can do, independent of device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Confirm,
    Pause,
    Save,
    Load,
    ToggleDebug,
}

impl InputAction {
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "move-up" => Self::MoveUp,
            "move-down" => Self::MoveDown,
            "move-left" => Self::MoveLeft,
            "move-right" => Self::MoveRight,
            "confirm" => Self::Confirm,
            "pause" => Self::Pause,
            "save" => Self::Save,
            "load" => Self::Load,
            "toggle-debug" => Self::ToggleDebug,
            _ => return None,
        })
    }
}

/// One physical source of an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Key(KeyCode),
    Pad(GamepadButton),
}

/// Which sources trigger which action.
#[derive(Debug, Clone, Resource)]
pub struct Bindings(pub BTreeMap<InputAction, Vec<Source>>);

impl Bindings {
    /// Parses a bindings file. Key and button names are Bevy's own variant
    /// names, resolved through reflection, so no name table lives in code.
    pub fn from_text(text: &str) -> Result<Self, String> {
        let mut map = BTreeMap::new();
        for (action, sources) in crate::kv::parse(text)? {
            let action = InputAction::from_name(action)
                .ok_or_else(|| format!("unknown action `{action}`"))?;
            let sources = sources
                .split_whitespace()
                .map(parse_source)
                .collect::<Result<Vec<_>, _>>()?;
            map.insert(action, sources);
        }
        Ok(Self(map))
    }

    /// The default bindings shipped with the game.
    pub fn defaults() -> Self {
        Self::from_text(include_str!("../data/input/bindings.txt")).expect("bindings.txt is valid")
    }
}

fn parse_source(source: &str) -> Result<Source, String> {
    let (device, name) = source
        .split_once(':')
        .ok_or_else(|| format!("expected `device:Name`, got `{source}`"))?;
    let variant = DynamicEnum::new(name, DynamicVariant::Unit);
    match device {
        "key" => KeyCode::from_reflect(&variant).map(Source::Key),
        "pad" => GamepadButton::from_reflect(&variant).map(Source::Pad),
        _ => return Err(format!("unknown device `{device}` in `{source}`")),
    }
    .ok_or_else(|| format!("unknown {device} `{name}`"))
}

/// The actions this frame.
#[derive(Debug, Clone, Default, Resource)]
pub struct ActionState {
    pressed: BTreeSet<InputAction>,
    just_pressed: BTreeSet<InputAction>,
    stick: Vec2,
}

impl ActionState {
    /// Whether the action is held.
    pub fn pressed(&self, action: InputAction) -> bool {
        self.pressed.contains(&action)
    }

    /// Whether the action started this frame.
    pub fn just_pressed(&self, action: InputAction) -> bool {
        self.just_pressed.contains(&action)
    }

    /// The move direction, x right and y up, length at most 1: the digital
    /// directions and the left stick combined.
    pub fn move_axis(&self) -> Vec2 {
        let axis = |neg, pos| self.pressed(pos) as i32 as f32 - self.pressed(neg) as i32 as f32;
        let digital = Vec2::new(
            axis(InputAction::MoveLeft, InputAction::MoveRight),
            axis(InputAction::MoveDown, InputAction::MoveUp),
        );
        (digital + self.stick).clamp_length_max(1.0)
    }
}

/// Reads the devices into [`ActionState`] at the start of every frame.
pub fn read_actions(
    bindings: Res<Bindings>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut state: ResMut<ActionState>,
) {
    let mut next = ActionState::default();
    for (action, sources) in &bindings.0 {
        for source in sources {
            let (held, started) = match *source {
                Source::Key(key) => (keys.pressed(key), keys.just_pressed(key)),
                Source::Pad(button) => (
                    gamepads.iter().any(|g| g.pressed(button)),
                    gamepads.iter().any(|g| g.just_pressed(button)),
                ),
            };
            if held {
                next.pressed.insert(*action);
            }
            if started {
                next.just_pressed.insert(*action);
            }
        }
    }
    next.stick = gamepads.iter().map(|g| g.left_stick()).sum();
    *state = next;
}

/// Adds the action layer.
pub fn plugin(app: &mut App) {
    app.insert_resource(Bindings::defaults())
        .init_resource::<ActionState>()
        .add_systems(PreUpdate, read_actions.after(bevy::input::InputSystems));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bindings_cover_every_action() {
        let bindings = Bindings::defaults();
        for action in [
            InputAction::MoveUp,
            InputAction::MoveDown,
            InputAction::MoveLeft,
            InputAction::MoveRight,
            InputAction::Confirm,
            InputAction::Pause,
            InputAction::Save,
            InputAction::Load,
            InputAction::ToggleDebug,
        ] {
            assert!(!bindings.0[&action].is_empty(), "{action:?} is unbound");
        }
        assert!(bindings.0[&InputAction::Confirm].contains(&Source::Key(KeyCode::Enter)));
        assert!(bindings.0[&InputAction::Pause].contains(&Source::Pad(GamepadButton::Start)));
    }

    #[test]
    fn rejects_unknown_names() {
        assert!(Bindings::from_text("confirm = key:NoSuchKey").is_err());
        assert!(Bindings::from_text("jump = key:Space").is_err());
    }

    #[test]
    fn move_axis_combines_and_clamps() {
        let mut state = ActionState::default();
        state.pressed.insert(InputAction::MoveRight);
        state.pressed.insert(InputAction::MoveUp);
        state.stick = Vec2::new(1.0, 0.0);
        assert!((state.move_axis().length() - 1.0).abs() < 1e-6);
    }
}
