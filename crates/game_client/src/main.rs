//! The game client: presentation, input, HUD, localisation.
//!
//! The client is a pure observer of the simulation. It reads through the
//! observation (ADR-0007) and writes back exactly one thing: the intent
//! (ADR-0002).
//!
//! | Module | Genre-neutral? |
//! | --- | --- |
//! | `screens`, `run`, `input`, `localisation`, `font`, `overlay`, `kv`, `saving`, `storage` | yes: keep, extend |
//! | `presentation` | no: the example game's look; replace with `game_sim::game` |

mod font;
mod input;
mod kv;
mod localisation;
mod overlay;
mod presentation;
mod run;
mod saving;
mod screens;
mod storage;

use bevy::prelude::*;

use game_sim::SimPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(font::plugin)
        .add_plugins(SimPlugin)
        .init_resource::<run::RunSetup>()
        .insert_resource(localisation::Localisation::english())
        .add_plugins((
            input::plugin,
            screens::plugin,
            presentation::plugin,
            overlay::plugin,
            saving::plugin,
        ))
        .run();
}
