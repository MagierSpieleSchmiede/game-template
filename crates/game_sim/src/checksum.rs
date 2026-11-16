//! A checksum of the simulation state: equal states hash equally on every
//! platform. Used by the determinism tests against a committed value, by
//! replays to prove a playback matches, and by rollback as desync detection.

use bevy::prelude::*;

use crate::snapshot::Snapshot;

/// The state hash of the world's current simulation state.
pub fn state_hash(world: &mut World) -> u64 {
    Snapshot::capture(world).hash()
}

/// FNV-1a over little-endian bytes: tiny, platform-independent, no crate.
/// The game feeds its state into it in `SimId` order, floats by their exact
/// bits (`f32::to_bits`).
#[derive(Debug, Clone)]
pub struct StateHasher(u64);

impl Default for StateHasher {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl StateHasher {
    /// Feeds a `u64`.
    pub fn write_u64(&mut self, value: u64) {
        for byte in value.to_le_bytes() {
            self.0 ^= byte as u64;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    /// Feeds an `i64`.
    pub fn write_i64(&mut self, value: i64) {
        self.write_u64(value as u64);
    }

    /// Feeds an `f32` by its exact bits.
    pub fn write_f32(&mut self, value: f32) {
        self.write_u64(value.to_bits() as u64);
    }

    /// The hash so far.
    pub fn finish(&self) -> u64 {
        self.0
    }
}
