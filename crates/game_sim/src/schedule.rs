//! The tick chain: every simulation system runs in exactly one of these sets
//! in `FixedUpdate`, and the sets run in this order every tick
//! (`docs/architecture/simulation-model.md`, invariant 2).

use bevy::ecs::schedule::IntoScheduleConfigs;
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

/// The whole tick chain, in order. `SimPlugin` adds this to `FixedUpdate`.
pub fn tick_chain() -> impl IntoScheduleConfigs<ScheduleSystem, ()> {
    (
        apply_intents.in_set(SimSet::Intent),
        integrate_movement.in_set(SimSet::Movement),
    )
        .chain()
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

/// Moves every unit by its velocity, clamped to the arena.
fn integrate_movement(balance: Res<Balance>, mut units: Query<&mut Unit>) {
    let delta = 1.0 / crate::TICKS_PER_SECOND as f32;
    for mut unit in &mut units {
        let step = unit.velocity * delta;
        unit.position += step;
        let radius = unit.position.truncate().length();
        if radius > balance.arena_radius {
            unit.position *= balance.arena_radius / radius;
        }
    }
}
