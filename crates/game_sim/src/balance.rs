//! The central balance resource (ADR-0010): every gameplay constant lives
//! here, never as a magic number inside a system.

use bevy::prelude::*;

/// All gameplay constants of the prototype, in one resource. Values are
/// per second where marked; the simulation converts them with
/// [`crate::TICKS_PER_SECOND`].
#[derive(Debug, Clone, Resource)]
pub struct Balance {
    /// Movement speed in metres per second.
    pub move_speed: f32,
    /// Radius of the arena floor in metres.
    pub arena_radius: f32,
}

impl Default for Balance {
    fn default() -> Self {
        Self {
            move_speed: 6.0,
            arena_radius: 20.0,
        }
    }
}
