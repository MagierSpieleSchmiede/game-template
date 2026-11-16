//! Scenarios: the complete start state of a simulation run, as data.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::rng::SimRng;

/// A scenario: everything a run needs to be reproducible — the seed for the
/// RNG and the game's setup. Same scenario, same intents → same end state.
#[derive(Debug, Clone, Default, PartialEq, Resource, Serialize, Deserialize)]
pub struct Scenario {
    /// Seed for [`SimRng`].
    pub seed: u64,
    /// What the game spawns at the start; defined by the game.
    pub setup: crate::game::Setup,
}

/// Starts the run from the scenario: seeds [`SimRng`] (the one place that
/// happens, for the client and the headless app alike), then lets the game
/// spawn its start state.
pub fn start_run(world: &mut World) {
    let scenario = world.resource::<Scenario>().clone();
    world.insert_resource(SimRng::seeded(scenario.seed));
    crate::game::spawn(world, &scenario.setup);
}
