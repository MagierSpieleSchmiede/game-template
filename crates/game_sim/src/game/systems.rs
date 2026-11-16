//! The example game's tick: its own sets inside `SimSet::Update`, in order.

use std::collections::BTreeMap;

use bevy::ecs::schedule::{InternedSystemSet, IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;

use super::{Action, Balance, Unit};
use crate::ids::SimId;
use crate::intent::TickIntents;

/// The example game's stages, in order. A typical chain for games with units
/// that fight; other genres use other stages.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArenaSet {
    /// Apply this tick's intents.
    Intent,
    /// Cooldowns, regeneration, attribute modifiers.
    Values,
    /// Move units and projectiles.
    Movement,
    /// Hits, effects, damage, death.
    Damage,
    /// Despawn spent entities.
    Cleanup,
}

impl ArenaSet {
    /// The stages in their fixed order.
    pub fn chain() -> impl IntoScheduleConfigs<InternedSystemSet, ()> {
        (
            ArenaSet::Intent,
            ArenaSet::Values,
            ArenaSet::Movement,
            ArenaSet::Damage,
            ArenaSet::Cleanup,
        )
            .chain()
    }
}

/// The systems, each in its stage.
pub fn systems() -> ScheduleConfigs<ScheduleSystem> {
    (
        apply_intents.in_set(ArenaSet::Intent),
        integrate_movement.in_set(ArenaSet::Movement),
    )
        .into_configs()
}

/// Applies this tick's intents: `Move` sets the unit's velocity until its
/// next intent, `Idle` stops it.
fn apply_intents(
    intents: Res<TickIntents>,
    balance: Res<Balance>,
    mut units: Query<(&SimId, &mut Unit)>,
) {
    let mut by_id: BTreeMap<SimId, Mut<Unit>> =
        units.iter_mut().map(|(id, unit)| (*id, unit)).collect();
    for intent in &intents.0 {
        let Some(unit) = by_id.get_mut(&intent.actor) else {
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
