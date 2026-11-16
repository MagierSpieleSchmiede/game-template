//! The observation: the one code path through which HUD, presentation, AI,
//! add-ons and the network read the simulation (ADR-0007).
//!
//! It returns plain data (no `Entity`, no Bevy handles), filtered by what the
//! viewer may see. The filter itself is the game's: `game::observe` decides
//! what a team sees.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::tick::SimTick;

/// A team: the unit of visibility. No code assumes exactly two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TeamId(pub u32);

/// Who is looking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Viewer {
    /// Sees everything: replays, spectators, debugging, the referee.
    Spectator,
    /// Sees what the team may see.
    Team(TeamId),
}

/// What a viewer sees at the end of a tick.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    /// The tick this observation was taken after.
    pub tick: SimTick,
    /// The game's view of its state, filtered for the viewer.
    pub game: crate::game::GameObservation,
}

/// Observes the world for `viewer`. Takes `&mut World` because queries need
/// it; nothing is changed.
pub fn observe(world: &mut World, viewer: Viewer) -> Observation {
    Observation {
        tick: *world.resource::<SimTick>(),
        game: crate::game::observe(world, viewer),
    }
}
