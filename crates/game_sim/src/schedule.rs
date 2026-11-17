//! The tick chain: every simulation system runs in exactly one of these sets
//! in `FixedUpdate`, and the sets run in this order every tick
//! (`docs/architecture/simulation-model.md`, invariant 1).

use bevy::ecs::schedule::{InternedSystemSet, IntoScheduleConfigs};
use bevy::prelude::*;

/// The core sets of a tick, in order. The game puts its systems in
/// [`SimSet::Update`] and may order them there with its own sets.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimSet {
    /// Count the tick, record the arrived intents for replays, check and
    /// freeze them.
    Input,
    /// The game's systems.
    Update,
    /// End-of-tick work of the core (none yet; checks and statistics go here).
    End,
}

/// The core sets in their fixed order. `SimPlugin` configures them in
/// `FixedUpdate`.
pub fn sim_sets() -> impl IntoScheduleConfigs<InternedSystemSet, ()> {
    (SimSet::Input, SimSet::Update, SimSet::End).chain()
}
