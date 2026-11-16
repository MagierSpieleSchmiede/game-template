//! The game client: presentation, input, HUD, localisation.
//!
//! The client is a pure observer of the simulation. It reads through the
//! observation (ADR-0007) and writes back exactly one thing: the intent
//! (ADR-0002).
//!
//! | Module | Genre-neutral? |
//! | --- | --- |
//! | `screens`, `input`, `localisation`, `overlay`, `kv` | yes: keep, extend |
//! | `presentation` | no: the example game's look; replace with `game_sim::game` |

mod input;
mod kv;
mod localisation;
mod overlay;
mod presentation;
mod screens;

use bevy::prelude::*;

use game_sim::SimPlugin;
use game_sim::scenario::Scenario;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimPlugin)
        .insert_resource(Scenario::default())
        .insert_resource(localisation::Localisation::english())
        .add_plugins((
            input::plugin,
            screens::plugin,
            presentation::plugin,
            overlay::plugin,
        ))
        .run();
}
