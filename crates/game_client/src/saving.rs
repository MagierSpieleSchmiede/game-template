//! Save and load (template default, ADR-0016): the `save` action stores the
//! running world as a [`SaveGame`] in the quicksave slot of the platform's
//! [`Storage`]; the `load` action restores it, from the menu too. A short
//! localised notice says what happened, including why a file was refused.
//!
//! Loading replaces the simulation state as a whole, between ticks: it is not
//! a gameplay write, so the rule "the presentation writes back only the
//! intent" holds. The presentation follows through a fresh observation.

use std::io;

use bevy::prelude::*;

use game_sim::file::FileError;
use game_sim::save::SaveGame;

use crate::input::{ActionState, InputAction};
use crate::localisation::Localisation;
use crate::screens::Screen;
use crate::storage::{Storage, platform_store};

/// The one slot the actions use.
pub const QUICKSAVE: &str = "quicksave";

/// How long a notice stays on screen, in real seconds (presentation time,
/// not simulation time).
const NOTICE_SECONDS: f64 = 3.0;

/// Why loading failed.
#[derive(Debug)]
pub enum LoadError {
    /// Nothing saved yet.
    NoSave,
    /// The store could not be read.
    Storage(io::Error),
    /// The bytes were read but refused (ADR-0017).
    Refused(FileError),
}

/// Saves the world's run to `slot`.
pub fn save_game(world: &mut World, slot: &str) -> io::Result<()> {
    let bytes = SaveGame::capture(world).to_bytes();
    world.resource_mut::<Storage>().0.write(slot, &bytes)
}

/// Loads the run in `slot` into the world and lets the presentation observe
/// it. A refused file leaves the world as it was.
pub fn load_game(world: &mut World, slot: &str) -> Result<(), LoadError> {
    let bytes = world
        .resource::<Storage>()
        .0
        .read(slot)
        .map_err(LoadError::Storage)?
        .ok_or(LoadError::NoSave)?;
    let save = SaveGame::from_bytes(&bytes).map_err(LoadError::Refused)?;
    save.restore(world);
    crate::presentation::observe_restored(world);
    Ok(())
}

/// The notice after saving.
pub fn save_notice(texts: &Localisation, result: &io::Result<()>) -> String {
    match result {
        Ok(()) => texts.text("save-done", &[]),
        Err(e) => texts.text("save-failed", &[("reason", &e.to_string())]),
    }
}

/// The notice after loading: one text per reason a file is refused.
pub fn load_notice(texts: &Localisation, result: &Result<(), LoadError>) -> String {
    let refused = |key| texts.text(key, &[]);
    let versions = |key, expected: &dyn ToString, found: &dyn ToString| {
        texts.text(
            key,
            &[
                ("expected", &expected.to_string()),
                ("found", &found.to_string()),
            ],
        )
    };
    match result {
        Ok(()) => texts.text("load-done", &[]),
        Err(LoadError::NoSave) => texts.text("load-none", &[]),
        Err(LoadError::Storage(e)) => texts.text("load-failed", &[("reason", &e.to_string())]),
        Err(LoadError::Refused(error)) => match error {
            FileError::NotAGameFile => refused("load-refused-not-a-game-file"),
            FileError::FormatVersion { expected, found } => {
                versions("load-refused-format-version", expected, found)
            }
            FileError::WrongKind { .. } => refused("load-refused-wrong-kind"),
            FileError::SimVersion { expected, found } => {
                versions("load-refused-sim-version", expected, found)
            }
            FileError::Corrupt(_) => refused("load-refused-corrupt"),
            FileError::HashMismatch { .. } => refused("load-refused-hash"),
        },
    }
}

/// The text in the notice and the real time it disappears.
#[derive(Debug, Default, Resource)]
struct Notice {
    text: String,
    until: f64,
}

#[derive(Debug, Component)]
struct NoticeText;

/// Adds save and load with the platform's store.
pub fn plugin(app: &mut App) {
    app.insert_resource(Storage(platform_store()))
        .init_resource::<Notice>()
        .add_systems(Startup, spawn_notice)
        .add_systems(Update, (save_and_load, show_notice).chain());
}

/// Runs the `save` and `load` actions. Saving needs a run on screen; loading
/// from the menu starts playing the loaded run.
fn save_and_load(world: &mut World) {
    let actions = world.resource::<ActionState>();
    let (save, load) = (
        actions.just_pressed(InputAction::Save),
        actions.just_pressed(InputAction::Load),
    );
    let screen = *world.resource::<State<Screen>>().get();
    let text = if save && screen != Screen::Menu {
        let result = save_game(world, QUICKSAVE);
        if let Err(e) = &result {
            warn!("saving failed: {e}");
        }
        save_notice(world.resource::<Localisation>(), &result)
    } else if load {
        let result = load_game(world, QUICKSAVE);
        match &result {
            Ok(()) if screen == Screen::Menu => {
                world
                    .resource_mut::<NextState<Screen>>()
                    .set(Screen::Playing);
            }
            Ok(()) => {}
            Err(e) => warn!("loading failed: {e:?}"),
        }
        load_notice(world.resource::<Localisation>(), &result)
    } else {
        return;
    };
    let until = world.resource::<Time<Real>>().elapsed_secs_f64() + NOTICE_SECONDS;
    world.insert_resource(Notice { text, until });
}

fn spawn_notice(mut commands: Commands) {
    commands.spawn((
        NoticeText,
        Text::default(),
        TextFont::from_font_size(20.0),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(24),
            left: px(24),
            ..default()
        },
        Visibility::Hidden,
    ));
}

fn show_notice(
    notice: Res<Notice>,
    time: Res<Time<Real>>,
    mut shown: Single<(&mut Text, &mut Visibility), With<NoticeText>>,
) {
    let (text, visibility) = &mut *shown;
    let visible = time.elapsed_secs_f64() < notice.until;
    let wanted = if visible {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    visibility.set_if_neq(wanted);
    if notice.is_changed() {
        text.0.clone_from(&notice.text);
    }
}

#[cfg(test)]
mod tests {
    use game_sim::checksum::state_hash;
    use game_sim::file::FileKind;
    use game_sim::headless::{headless_app, step_ticks};
    use game_sim::scenario::Scenario;

    use super::*;
    use crate::presentation::Frames;
    use crate::run::RunSetup;
    use crate::storage::MemoryStore;

    fn app() -> App {
        let mut app = headless_app(Scenario::default());
        app.insert_resource(Storage(Box::new(MemoryStore::default())))
            .insert_resource(Frames {
                hash_wanted: true,
                ..default()
            })
            .init_resource::<RunSetup>();
        app
    }

    #[test]
    fn loading_continues_the_saved_run() {
        let mut app = app();
        step_ticks(&mut app, 30);
        let world = app.world_mut();
        save_game(world, QUICKSAVE).unwrap();
        let saved = state_hash(world);
        step_ticks(&mut app, 45);
        let world = app.world_mut();
        assert_ne!(state_hash(world), saved);

        load_game(world, QUICKSAVE).unwrap();
        assert_eq!(state_hash(world), saved);
        let frames = world.resource::<Frames>();
        assert_eq!(
            frames.hash,
            Some(saved),
            "the overlay shows the loaded state"
        );
        assert_eq!(frames.current.as_ref().unwrap().tick.0, 30);
        assert!(
            frames.previous.is_none(),
            "no interpolation across the load"
        );

        // The loaded run goes on exactly like the saved one did.
        let mut original = headless_app(Scenario::default());
        step_ticks(&mut original, 40);
        step_ticks(&mut app, 10);
        assert_eq!(
            state_hash(app.world_mut()),
            state_hash(original.world_mut())
        );
    }

    #[test]
    fn missing_and_refused_saves_leave_the_world_alone() {
        let mut app = app();
        step_ticks(&mut app, 5);
        let world = app.world_mut();
        let before = state_hash(world);
        assert!(matches!(
            load_game(world, QUICKSAVE),
            Err(LoadError::NoSave)
        ));
        world
            .resource_mut::<Storage>()
            .0
            .write(QUICKSAVE, b"not a save")
            .unwrap();
        assert!(matches!(
            load_game(world, QUICKSAVE),
            Err(LoadError::Refused(FileError::NotAGameFile))
        ));
        assert_eq!(state_hash(world), before);
    }

    #[test]
    fn every_outcome_has_a_localised_notice() {
        let texts = Localisation::english();
        let io_error = || io::Error::other("disk full");
        let mut notices = vec![
            save_notice(&texts, &Ok(())),
            save_notice(&texts, &Err(io_error())),
            load_notice(&texts, &Ok(())),
            load_notice(&texts, &Err(LoadError::NoSave)),
            load_notice(&texts, &Err(LoadError::Storage(io_error()))),
        ];
        for error in [
            FileError::NotAGameFile,
            FileError::FormatVersion {
                expected: 2,
                found: 1,
            },
            FileError::WrongKind {
                expected: FileKind::Save,
                found: FileKind::Replay,
            },
            FileError::SimVersion {
                expected: 7,
                found: 6,
            },
            FileError::Corrupt("eof".into()),
            FileError::HashMismatch {
                expected: 1,
                found: 2,
            },
        ] {
            notices.push(load_notice(&texts, &Err(LoadError::Refused(error))));
        }
        for notice in &notices {
            assert!(
                !notice.starts_with("save-") && !notice.starts_with("load-"),
                "en.txt lacks the key `{notice}`"
            );
            assert!(!notice.contains('{'), "unfilled placeholder in `{notice}`");
        }
        assert!(notices[1].contains("disk full"));
        assert!(notices[8].contains('6') && notices[8].contains('7'));
    }
}
