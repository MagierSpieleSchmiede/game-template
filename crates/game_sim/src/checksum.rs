//! A checksum of the simulation state: equal states hash equally on every
//! platform. Used by the determinism test against a committed value, and the
//! basis for desync detection once the network exists (ADR-0006).

use bevy::prelude::*;

use crate::{ids::SimId, rng::SimRng, unit::Unit};

/// FNV-1a over every unit in `SimId` order (id, position, velocity, health,
/// floats by their exact bits) and the RNG's next draw. Takes `&mut World`
/// because a query needs it; the state is not changed.
pub fn state_hash(world: &mut World) -> u64 {
    let mut units: Vec<(SimId, Unit)> = world
        .query::<(&SimId, &Unit)>()
        .iter(world)
        .map(|(id, unit)| (*id, unit.clone()))
        .collect();
    units.sort_by_key(|(id, _)| *id);

    let mut hash = Fnv1a::default();
    for (id, unit) in &units {
        hash.write(id.0 as u64);
        for v in [unit.position, unit.velocity] {
            for c in v.to_array() {
                hash.write(c.to_bits() as u64);
            }
        }
        hash.write(unit.health as u64);
    }
    hash.write(world.resource::<SimRng>().clone().next_u64());
    hash.0
}

struct Fnv1a(u64);

impl Default for Fnv1a {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Fnv1a {
    fn write(&mut self, value: u64) {
        for byte in value.to_le_bytes() {
            self.0 ^= byte as u64;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}
