//! Stable simulation IDs (ADR-0008).
//!
//! Entity indices are ephemeral and unordered; the simulation orders by
//! `SimId` and serialises by `SimId`, never by `Entity`.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A stable, dense simulation ID assigned at spawn time in deterministic
/// order. Everything that is ordered, hashed or serialised in the simulation
/// keys on this, never on `Entity`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Serialize, Deserialize,
)]
pub struct SimId(pub i64);

impl SimId {
    /// The next free ID from the spawn counter.
    pub fn next(counter: &mut i64) -> Self {
        *counter += 1;
        Self(*counter)
    }
}
