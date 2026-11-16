//! The intent: the one thing the presentation writes back (ADR-0002).
//!
//! An intent is plain, serialisable data — no Bevy types, no entity handles —
//! so it can cross the network, feed a replay, be produced by an AI or an
//! add-on, and be re-applied identically in a rollback (ADR-0006, ADR-0007).
//! What an intent can *say* is the game's [`Action`]; who says it is a
//! [`SimId`].

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::game::Action;
use crate::ids::SimId;

/// What one actor (a player, an AI, an add-on acting for one) wants
/// from the next tick on. Plain data only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    /// The acting entity's simulation ID.
    pub actor: SimId,
    /// What it wants; defined by the game.
    pub action: Action,
}

/// The intents collected for the next tick, in arrival order. Anyone who acts
/// (input, AI, network) pushes here.
#[derive(Debug, Clone, Default, Resource)]
pub struct IntentBuffer {
    /// Intents for the upcoming tick.
    pub intents: Vec<Intent>,
}

impl IntentBuffer {
    /// Queues an intent for the next tick.
    pub fn push(&mut self, intent: Intent) {
        self.intents.push(intent);
    }
}

/// The intents of the current tick, frozen at its start and ordered by actor
/// (arrival order is kept per actor). Game systems read this, never the
/// buffer: arrival order differs between machines, `SimId` order does not.
#[derive(Debug, Clone, Default, Resource, PartialEq, Serialize, Deserialize)]
pub struct TickIntents(pub Vec<Intent>);

/// Moves the buffered intents into [`TickIntents`] for this tick.
pub fn freeze_intents(mut buffer: ResMut<IntentBuffer>, mut frozen: ResMut<TickIntents>) {
    let mut intents = std::mem::take(&mut buffer.intents);
    intents.sort_by_key(|intent| intent.actor);
    frozen.0 = intents;
}
