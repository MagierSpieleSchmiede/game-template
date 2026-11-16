//! The one seeded RNG resource for all randomness in the simulation (ADR-0004).
//!
//! Xoshiro256\*\* seeded through SplitMix64: small, fast, no external crate,
//! and fully deterministic across platforms. Everything that needs randomness
//! draws from `SimRng`; nothing else in the simulation may generate random
//! values (enforced by `clippy.toml`).

use bevy::prelude::*;

/// The one seeded RNG resource. Created from the scenario seed; rollback
/// restores it by replaying ticks from a snapshot.
#[derive(Debug, Clone, Resource)]
pub struct SimRng {
    state: [u64; 4],
}

impl SimRng {
    /// Seeds the generator deterministically from a single u64.
    pub fn seeded(seed: u64) -> Self {
        let mut sm = SplitMix64 { state: seed };
        Self {
            state: [sm.next(), sm.next(), sm.next(), sm.next()],
        }
    }

    /// Next raw u64. The only randomness source in the simulation.
    pub fn next_u64(&mut self) -> u64 {
        let result = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);
        result
    }

    /// Uniform value in `[0, bound)`; `bound` must be greater than zero.
    ///
    /// Rejection sampling keeps every outcome equally likely, so the same
    /// seed produces the same sequence on every platform.
    pub fn below(&mut self, bound: u64) -> u64 {
        assert!(bound > 0, "bound must be greater than zero");
        let zone = u64::MAX - (u64::MAX % bound) - 1;
        loop {
            let v = self.next_u64();
            if v <= zone {
                return v % bound;
            }
        }
    }
}

impl Default for SimRng {
    fn default() -> Self {
        Self::seeded(0)
    }
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = SimRng::seeded(42);
        let mut b = SimRng::seeded(42);
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_diverge() {
        let mut a = SimRng::seeded(1);
        let mut b = SimRng::seeded(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn below_stays_in_range() {
        let mut rng = SimRng::seeded(7);
        for _ in 0..1000 {
            assert!(rng.below(6) < 6);
        }
    }
}
