//! Units: everything that acts in the arena — players, creatures, bosses.

use bevy::prelude::*;

use crate::ids::SimId;

/// One simulated unit. Simulation state only: no meshes, materials, lights or
/// UI ever live here; the presentation mirrors this state as an observer.
#[derive(Debug, Clone, Component)]
pub struct Unit {
    /// The stable ID of this unit; ordering and serialisation key on it.
    pub id: SimId,
    /// Position on the arena floor, in metres.
    pub position: Vec3,
    /// Velocity for this tick, in metres per second.
    pub velocity: Vec3,
    /// Remaining health in thousandths (`i64`, so small rates divide evenly).
    pub health: i64,
}

impl Unit {
    /// The duration of one tick in seconds: the fixed step of the simulation,
    /// never the frame delta.
    pub fn tick_delta(&self) -> f32 {
        1.0 / crate::TICKS_PER_SECOND as f32
    }
}
