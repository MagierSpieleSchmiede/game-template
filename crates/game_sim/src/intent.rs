//! The intent: the one thing the presentation writes back (ADR-0002).
//!
//! An intent is plain, serialisable data — no Bevy types, no entity handles —
//! so it can cross the network, feed a replay, be produced by an AI or an
//! add-on, and be re-applied identically in a rollback (ADR-0006, ADR-0007).
//! What an intent can *say* is the game's [`Action`]; who says it is a
//! [`PlayerId`], about an actor named by its [`SimId`].
//!
//! Intents come from outside the simulation, so they are checked before any
//! game system sees them: the game's [`crate::game::validate`] (well-formed
//! values, no NaN) and [`Control`] (the player controls the actor). Rejected
//! intents never reach the game; they are listed in [`RejectedIntents`].

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::control::{Control, PlayerId};
pub use crate::game::Action;
use crate::ids::SimId;

/// What one player wants one actor to do from the next tick on. Plain data
/// only. Over the network, the server sets `player` from the connection, never
/// from the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    /// Who sends it.
    pub player: PlayerId,
    /// The acting entity's simulation ID.
    pub actor: SimId,
    /// What it wants; defined by the game.
    pub action: Action,
}

/// The intents collected for the next tick, in arrival order. Anyone who acts
/// (input, AI, network) pushes here.
#[derive(Debug, Clone, Default, Resource)]
pub struct IntentBuffer {
    /// Intents for the upcoming tick.
    pub intents: Vec<Intent>,
}

impl IntentBuffer {
    /// Queues an intent for the next tick.
    pub fn push(&mut self, intent: Intent) {
        self.intents.push(intent);
    }
}

/// The accepted intents of the current tick, frozen at its start and ordered
/// by actor (arrival order is kept per actor). Game systems read this, never
/// the buffer: arrival order differs between machines, `SimId` order does not.
#[derive(Debug, Clone, Default, Resource, PartialEq, Serialize, Deserialize)]
pub struct TickIntents(pub Vec<Intent>);

/// Why an intent was not applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rejection {
    /// The game's `validate` refused the action (NaN, out of range, …).
    InvalidAction,
    /// The sending player does not control the actor.
    NotInControl,
}

/// The intents rejected at the start of the current tick, with the reason. A
/// server reads this to tell bugs from cheating; the simulation ignores it.
#[derive(Debug, Clone, Default, Resource, PartialEq)]
pub struct RejectedIntents(pub Vec<(Intent, Rejection)>);

/// Moves the buffered intents into [`TickIntents`] for this tick, keeping only
/// valid intents from the actor's controller.
pub fn freeze_intents(
    mut buffer: ResMut<IntentBuffer>,
    control: Res<Control>,
    mut frozen: ResMut<TickIntents>,
    mut rejected: ResMut<RejectedIntents>,
) {
    rejected.0.clear();
    let mut accepted = Vec::new();
    for intent in std::mem::take(&mut buffer.intents) {
        if !crate::game::validate(&intent.action) {
            rejected.0.push((intent, Rejection::InvalidAction));
        } else if control.controller(intent.actor) != Some(intent.player) {
            rejected.0.push((intent, Rejection::NotInControl));
        } else {
            accepted.push(intent);
        }
    }
    accepted.sort_by_key(|intent| intent.actor);
    frozen.0 = accepted;
}
