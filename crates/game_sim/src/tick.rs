//! The tick counter: which tick the simulation is in.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The number of the last tick simulated; 0 before the first tick. Part of
/// the simulation state: snapshots, replays and rollback key on it.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Resource,
    Serialize,
    Deserialize,
)]
pub struct SimTick(pub u64);

/// Counts the tick; the first system of every tick.
pub fn advance_tick(mut tick: ResMut<SimTick>) {
    tick.0 += 1;
}
