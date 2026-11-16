//! Stable simulation IDs (ADR-0008).
//!
//! Entity indices are ephemeral and unordered; the simulation orders by
//! `SimId` and serialises by `SimId`, never by `Entity`.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A stable, dense simulation ID assigned at spawn time in deterministic
/// order. Everything that is ordered, hashed or serialised in the simulation
/// keys on this, never on `Entity`. Every simulation entity carries one as its
/// own component.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Serialize, Deserialize,
)]
pub struct SimId(pub i64);

/// The spawn counter: the last `SimId` handed out. A resource, so every spawn
/// in the run (scenario, projectiles, respawns) draws from the same sequence
/// and a rollback snapshot restores it together with the world.
#[derive(Debug, Clone, Default, Resource)]
pub struct SimIdCounter(i64);

impl SimIdCounter {
    /// The last ID handed out; 0 before the first.
    pub fn last(&self) -> i64 {
        self.0
    }

    /// The next free ID.
    pub fn allocate(&mut self) -> SimId {
        self.0 += 1;
        SimId(self.0)
    }
}
