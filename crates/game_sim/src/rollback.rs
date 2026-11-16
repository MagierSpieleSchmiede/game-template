//! Rollback: the template-default netcode (ADR-0006), headless and without a
//! transport yet.
//!
//! A [`RollbackSession`] simulates ahead with the intents it has. When an
//! intent for a past tick arrives late, it restores the snapshot from before
//! that tick and re-simulates to the present with the corrected intents. The
//! result must equal a run in which the intent arrived on time; the tests
//! check exactly that, which also proves snapshots are complete.
//!
//! A game that chooses lockstep, client-server prediction or no network at
//! all deletes this module (and its test); nothing else depends on it.

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::intent::{Intent, IntentBuffer};
use crate::snapshot::Snapshot;
use crate::tick::SimTick;

/// One simulated tick: the state before it and the intents it ran with.
#[derive(Debug, Clone)]
struct Frame {
    tick: SimTick,
    before: Snapshot,
    intents: Vec<Intent>,
}

/// An intent arrived for a tick older than the rollback window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooLate {
    /// The oldest tick that can still be corrected.
    pub oldest: Option<SimTick>,
}

/// A simulation that can rewind up to `window` ticks.
pub struct RollbackSession {
    app: App,
    frames: VecDeque<Frame>,
    window: usize,
}

impl RollbackSession {
    /// Wraps a headless app that has not been stepped outside this session.
    pub fn new(app: App, window: usize) -> Self {
        assert!(
            window > 0,
            "the rollback window must hold at least one tick"
        );
        Self {
            app,
            frames: VecDeque::with_capacity(window),
            window,
        }
    }

    /// The app, for reading (observation, state hash).
    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    /// The last simulated tick.
    pub fn tick(&self) -> SimTick {
        *self.app.world().resource::<SimTick>()
    }

    /// Simulates the next tick with the intents known now.
    pub fn advance(&mut self, intents: Vec<Intent>) {
        let before = Snapshot::capture(self.app.world_mut());
        let tick = SimTick(before.tick.0 + 1);
        Self::run(&mut self.app, &intents);
        self.frames.push_back(Frame {
            tick,
            before,
            intents,
        });
        while self.frames.len() > self.window {
            self.frames.pop_front();
        }
    }

    /// Adds an intent that belonged to the past tick `tick`, then rewinds to
    /// before that tick and re-simulates to the present. Returns how many
    /// ticks were re-simulated.
    pub fn insert_late(&mut self, tick: SimTick, intent: Intent) -> Result<usize, TooLate> {
        let Some(start) = self.frames.iter().position(|f| f.tick == tick) else {
            return Err(TooLate {
                oldest: self.frames.front().map(|f| f.tick),
            });
        };
        self.frames[start].intents.push(intent);
        self.frames[start].before.restore(self.app.world_mut());
        for frame in self.frames.iter_mut().skip(start) {
            frame.before = Snapshot::capture(self.app.world_mut());
            Self::run(&mut self.app, &frame.intents);
        }
        Ok(self.frames.len() - start)
    }

    fn run(app: &mut App, intents: &[Intent]) {
        app.world_mut().resource_mut::<IntentBuffer>().intents = intents.to_vec();
        app.update();
    }
}
