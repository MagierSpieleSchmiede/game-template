//! Snapshots: the whole simulation state as one value, and back.
//!
//! The base of save games, rollback (ADR-0006), desync checks and debugging.
//! A snapshot is complete when restoring it and re-running the same intents
//! gives the same state hash as the original run; `tests/snapshot.rs` checks
//! that, so any state the game forgets to capture shows up as a red test.

use bevy::prelude::*;

use crate::checksum::StateHasher;
use crate::control::Control;
use crate::ids::{SimId, SimIdCounter};
use crate::intent::{IntentBuffer, RejectedIntents, TickIntents};
use crate::rng::SimRng;
use crate::tick::SimTick;

/// The complete simulation state between two ticks. Per-tick scratch data
/// (pending, frozen and rejected intents) is not part of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
    /// The last simulated tick.
    pub tick: SimTick,
    /// The RNG, mid-sequence.
    pub rng: SimRng,
    /// The spawn counter.
    pub ids: SimIdCounter,
    /// Who controls which actor.
    pub control: Control,
    /// Everything the game simulates.
    pub game: crate::game::GameState,
}

impl Snapshot {
    /// Captures the current state. Takes `&mut World` because queries need
    /// it; nothing is changed.
    pub fn capture(world: &mut World) -> Self {
        Self {
            tick: *world.resource::<SimTick>(),
            rng: world.resource::<SimRng>().clone(),
            ids: world.resource::<SimIdCounter>().clone(),
            control: world.resource::<Control>().clone(),
            game: crate::game::capture(world),
        }
    }

    /// Restores this state: every simulation entity (everything with a
    /// [`SimId`]) is despawned and the game respawns its own from the
    /// snapshot. Pending intents are dropped; whoever restores supplies the
    /// intents to re-run.
    pub fn restore(&self, world: &mut World) {
        clear_simulation(world);
        world.insert_resource(self.tick);
        world.insert_resource(self.rng.clone());
        world.insert_resource(self.ids.clone());
        world.insert_resource(self.control.clone());
        crate::game::restore(world, &self.game);
    }

    /// The state hash of this snapshot (see [`crate::checksum`]).
    pub fn hash(&self) -> u64 {
        let mut hasher = StateHasher::default();
        hasher.write_u64(self.tick.0);
        for word in self.rng.state() {
            hasher.write_u64(word);
        }
        hasher.write_i64(self.ids.last());
        for (actor, player) in self.control.iter() {
            hasher.write_i64(actor.0);
            hasher.write_u64(player.0 as u64);
        }
        self.game.hash_into(&mut hasher);
        hasher.finish()
    }
}

/// Despawns every simulation entity (everything with a [`SimId`]) and drops
/// the per-tick scratch data (pending, frozen and rejected intents): what a
/// restore or a new run replaces.
pub(crate) fn clear_simulation(world: &mut World) {
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<SimId>>()
        .iter(world)
        .collect();
    for entity in entities {
        world.despawn(entity);
    }
    world.insert_resource(RejectedIntents::default());
    world.insert_resource(IntentBuffer::default());
    world.insert_resource(TickIntents::default());
}
