//! Scenarios: the complete start state of a simulation run, as data.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::control::Control;
use crate::ids::SimIdCounter;
use crate::rng::SimRng;
use crate::snapshot::clear_simulation;
use crate::tick::SimTick;

/// A scenario: everything a run needs to be reproducible — the seed for the
/// RNG and the game's setup. Same scenario, same intents → same end state.
#[derive(Debug, Clone, Default, PartialEq, Resource, Serialize, Deserialize)]
pub struct Scenario {
    /// Seed for [`SimRng`].
    pub seed: u64,
    /// What the game spawns at the start; defined by the game.
    pub setup: crate::game::Setup,
}

/// Starts the run from the scenario: clears whatever an earlier run left
/// (simulation entities, pending intents, tick, IDs, control), seeds
/// [`SimRng`] (the one place that happens, for the client and the headless
/// app alike), then lets the game spawn its start state. Starting again in a
/// used world gives the same state as a fresh app.
pub fn start_run(world: &mut World) {
    let scenario = world.resource::<Scenario>().clone();
    clear_simulation(world);
    world.insert_resource(SimTick::default());
    world.insert_resource(SimIdCounter::default());
    world.insert_resource(Control::default());
    world.insert_resource(SimRng::seeded(scenario.seed));
    crate::game::spawn(world, &scenario.setup);
}
