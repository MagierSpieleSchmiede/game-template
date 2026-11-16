//! The intent: the one thing the presentation writes back (ADR-0002).
//!
//! An intent is plain, serialisable data — no Bevy types, no entity handles —
//! so it can cross the network, feed a replay, be produced by an AI or an
//! add-on, and be re-applied identically in a rollback (ADR-0007).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The intents collected for the next tick, in arrival order.
#[derive(Debug, Clone, Default, Resource)]
pub struct IntentBuffer {
    /// Intents for the upcoming tick, applied by the tick chain in order.
    pub intents: Vec<Intent>,
}

impl IntentBuffer {
    /// Queues an intent for the next tick.
    pub fn push(&mut self, intent: Intent) {
        self.intents.push(intent);
    }

    /// Takes all queued intents, leaving the buffer empty for the next tick.
    pub fn take(&mut self) -> Vec<Intent> {
        std::mem::take(&mut self.intents)
    }
}

/// What a player (or AI, or add-on) wants to do in one tick. Plain data only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    /// The acting unit's simulation ID.
    pub unit: crate::ids::SimId,
    /// What the unit wants to do; extend this enum as the game grows.
    pub action: Action,
}

/// The kinds of things a unit can want in a tick. Keep variants stable once
/// they travel over the network (see `docs/architecture/serialization-ids.md`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Move along the given direction, magnitude in `[0, 1]`.
    Move { x: f32, z: f32 },
    /// Do nothing this tick.
    Idle,
}
