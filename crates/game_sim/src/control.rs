//! Who controls what: every actor that may receive intents is controlled by
//! one player, and only that player's intents for it are applied. Without
//! this, any client (or add-on) could steer anyone's actor
//! (`docs/architecture/security.md`).
//!
//! Part of the simulation state: snapshots and the state hash include it, so
//! a change of control (a unit handed over, a player leaving) rolls back and
//! replays like everything else.

use std::collections::BTreeMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ids::SimId;

/// A player: a person, an AI or an add-on that sends intents. Players are
/// numbered by the scenario or the lobby; no code assumes exactly two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u32);

/// Which player controls which actor.
#[derive(Debug, Clone, Default, PartialEq, Resource)]
pub struct Control(BTreeMap<SimId, PlayerId>);

impl Control {
    /// Gives `actor` to `player`, replacing any earlier controller.
    pub fn assign(&mut self, actor: SimId, player: PlayerId) {
        self.0.insert(actor, player);
    }

    /// Takes `actor` away from whoever controls it.
    pub fn release(&mut self, actor: SimId) {
        self.0.remove(&actor);
    }

    /// The player controlling `actor`, if any.
    pub fn controller(&self, actor: SimId) -> Option<PlayerId> {
        self.0.get(&actor).copied()
    }

    /// Every (actor, player) pair, in `SimId` order.
    pub fn iter(&self) -> impl Iterator<Item = (SimId, PlayerId)> + '_ {
        self.0.iter().map(|(a, p)| (*a, *p))
    }
}
