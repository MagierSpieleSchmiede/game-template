//! The tick chain: every simulation system runs in exactly one of these sets
//! in `FixedUpdate`, and the sets run in this order every tick
//! (`docs/architecture/simulation-model.md`, invariant 2).

use bevy::ecs::schedule::{InternedSystemSet, IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;

use crate::{balance::Balance, ids::SimId, intent::Action, intent::IntentBuffer, unit::Unit};

/// The sets of the tick chain, in order. Extend as the game grows; never run
/// gameplay systems outside these sets.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimSet {
    /// Freeze each unit's intent for this tick.
    Intent,
    /// Cooldowns, regeneration, attribute modifiers.
    Values,
    /// Move units and projectiles.
    Movement,
    /// Hits, effects, damage, death.
    Damage,
    /// Who sees what.
    Visibility,
    /// Despawn spent entities.
    Cleanup,
}

/// The sets in their fixed order. `SimPlugin` configures these in
/// `FixedUpdate`, so a system added to any set runs after every earlier set,
/// whichever sets are populated.
pub fn sim_sets() -> impl IntoScheduleConfigs<InternedSystemSet, ()> {
    (
        SimSet::Intent,
        SimSet::Values,
        SimSet::Movement,
        SimSet::Damage,
        SimSet::Visibility,
        SimSet::Cleanup,
    )
        .chain()
}

/// The systems of the tick chain, each in its set. `SimPlugin` adds these to
/// `FixedUpdate`; their order comes from [`sim_sets`].
pub fn tick_chain() -> ScheduleConfigs<ScheduleSystem> {
    (
        apply_intents.in_set(SimSet::Intent),
        integrate_movement.in_set(SimSet::Movement),
    )
        .into_configs()
}

/// Applies the queued intents: each `Move` sets the unit's velocity for this
/// tick; everything else leaves the unit as it is.
fn apply_intents(
    mut buffer: ResMut<IntentBuffer>,
    balance: Res<Balance>,
    mut units: Query<(&SimId, &mut Unit)>,
) {
    let intents = buffer.take();
    for intent in intents {
        let Some((_, mut unit)) = units.iter_mut().find(|(id, _)| **id == intent.unit) else {
            continue;
        };
        unit.velocity = match intent.action {
            Action::Move { x, z } => {
                let dir = Vec3::new(x, 0.0, z);
                let dir = if dir.length_squared() > 1.0 {
                    dir.normalize()
                } else {
                    dir
                };
                dir * balance.move_speed
            }
            Action::Idle => Vec3::ZERO,
        };
    }
}

/// Moves every unit by its velocity, clamped to the arena disc on the x/z
/// floor (y is up and is left alone).
fn integrate_movement(balance: Res<Balance>, mut units: Query<&mut Unit>) {
    for mut unit in &mut units {
        let step = unit.velocity * crate::TICK_SECONDS;
        unit.position += step;
        let floor = Vec2::new(unit.position.x, unit.position.z);
        let radius = floor.length();
        if radius > balance.arena_radius {
            let clamped = floor * (balance.arena_radius / radius);
            unit.position.x = clamped.x;
            unit.position.z = clamped.y;
        }
    }
}
