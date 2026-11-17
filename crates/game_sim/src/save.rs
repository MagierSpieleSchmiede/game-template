//! Save games: the scenario and a snapshot of the whole simulation state,
//! with its state hash, as a file (ADR-0017).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::file::{self, FileError, FileKind};
use crate::headless::headless_app;
use crate::scenario::Scenario;
use crate::snapshot::Snapshot;

/// A saved run: enough to continue it exactly where it was saved.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGame {
    /// The run's start state (seed and setup).
    pub scenario: Scenario,
    /// The state when saved.
    pub snapshot: Snapshot,
    /// `snapshot.hash()`, checked on load.
    pub state_hash: u64,
}

impl SaveGame {
    /// Saves the world's current run.
    pub fn capture(world: &mut World) -> Self {
        let snapshot = Snapshot::capture(world);
        Self {
            scenario: world.resource::<Scenario>().clone(),
            state_hash: snapshot.hash(),
            snapshot,
        }
    }

    /// The save as file bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        file::encode(FileKind::Save, self)
    }

    /// Reads a save, refusing other files, other versions and damaged or
    /// edited states.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FileError> {
        let save: Self = file::decode(FileKind::Save, bytes)?;
        let found = save.snapshot.hash();
        if found != save.state_hash {
            return Err(FileError::HashMismatch {
                expected: save.state_hash,
                found,
            });
        }
        Ok(save)
    }

    /// Continues the saved run in `world`: its scenario becomes the world's,
    /// its snapshot replaces the simulation state.
    pub fn restore(&self, world: &mut World) {
        world.insert_resource(self.scenario.clone());
        self.snapshot.restore(world);
    }

    /// Continues the saved run in a fresh headless app.
    pub fn headless_app(&self) -> App {
        let mut app = headless_app(self.scenario.clone());
        self.restore(app.world_mut());
        app
    }
}
