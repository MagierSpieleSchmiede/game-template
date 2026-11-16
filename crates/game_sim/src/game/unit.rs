//! Units: everything that acts in the arena — players, creatures, bosses.

use bevy::prelude::*;

/// One simulated unit. Simulation state only: no meshes, materials, lights or
/// UI ever live here; the presentation mirrors this state as an observer.
/// Spawned together with its [`crate::ids::SimId`].
#[derive(Debug, Clone, PartialEq, Component)]
pub struct Unit {
    /// Position on the arena floor (the x/z plane, y up), in metres.
    pub position: Vec3,
    /// Velocity in metres per second; set by the latest `Move`/`Idle` intent
    /// and kept until the next one.
    pub velocity: Vec3,
    /// Remaining health in thousandths (`i64`, so small rates divide evenly).
    pub health: i64,
}
