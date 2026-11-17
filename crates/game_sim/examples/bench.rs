//! The headless benchmark (`docs/testing/benchmarks.md`): how fast the
//! simulation ticks for scenarios of several sizes, and what snapshots,
//! state hashes and a rollback cost. Rollback restores a snapshot and
//! re-simulates several ticks inside one frame, so these numbers bound the
//! rollback window.
//!
//! ```sh
//! cargo run -p game_sim --release --example bench
//! cargo run -p game_sim --release --example bench -- --units 2,100,1000 --ticks 600 --runs 7 --window 8
//! ```
//!
//! Every unit gets a move intent every tick (the worst case for intents),
//! turning a quarter each second so units keep hitting the arena edge. Each
//! figure is the median over `--runs` runs, with the spread (min–max) beside
//! it. Wall-clock time is fine here: this is a measuring tool, not simulation
//! code. A new game replaces [`intents_for`] with a load typical for it.

use std::hint::black_box;
use std::time::{Duration, Instant};

use game_sim::control::PlayerId;
use game_sim::game::{Action, Setup};
use game_sim::headless::headless_app;
use game_sim::ids::SimId;
use game_sim::intent::{Intent, IntentBuffer};
use game_sim::rollback::RollbackSession;
use game_sim::scenario::Scenario;
use game_sim::snapshot::Snapshot;
use game_sim::tick::SimTick;

/// What to measure.
#[derive(Debug, Clone, PartialEq)]
struct Config {
    /// Scenario sizes, one table row each.
    units: Vec<u32>,
    /// Ticks per run of the tick measurement.
    ticks: u32,
    /// Runs per figure; the median is reported.
    runs: u32,
    /// Ticks re-simulated by the measured rollback.
    window: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            units: vec![2, 100, 1000],
            ticks: 600,
            runs: 7,
            window: 8,
        }
    }
}

impl Config {
    /// Reads `--units a,b,c`, `--ticks n`, `--runs n`, `--window n`; anything
    /// left out keeps its default.
    fn from_args(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut config = Self::default();
        let mut args = args.into_iter();
        while let Some(flag) = args.next() {
            let value = args
                .next()
                .ok_or_else(|| format!("`{flag}` needs a value"))?;
            let number = |v: &str| {
                v.parse::<u32>()
                    .ok()
                    .filter(|n| *n > 0)
                    .ok_or_else(|| format!("`{flag}` expects positive numbers, got `{v}`"))
            };
            match flag.as_str() {
                "--units" => {
                    config.units = value.split(',').map(number).collect::<Result<_, _>>()?
                }
                "--ticks" => config.ticks = number(&value)?,
                "--runs" => config.runs = number(&value)?,
                "--window" => config.window = number(&value)? as usize,
                _ => return Err(format!("unknown flag `{flag}`")),
            }
        }
        Ok(config)
    }
}

/// Median and spread of one measured figure.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Stats {
    median: Duration,
    min: Duration,
    max: Duration,
}

impl Stats {
    fn of(mut samples: Vec<Duration>) -> Self {
        assert!(!samples.is_empty(), "no samples");
        samples.sort();
        let n = samples.len();
        let median = if n % 2 == 1 {
            samples[n / 2]
        } else {
            (samples[n / 2 - 1] + samples[n / 2]) / 2
        };
        Self {
            median,
            min: samples[0],
            max: samples[n - 1],
        }
    }

    /// Runs `measure` `runs` times.
    fn measure(runs: u32, mut measure: impl FnMut() -> Duration) -> Self {
        Self::of((0..runs).map(|_| measure()).collect())
    }
}

fn scenario(units: u32) -> Scenario {
    Scenario {
        seed: 7,
        setup: Setup { unit_count: units },
    }
}

/// The intents for `tick`: every unit moves, the direction turning a quarter
/// every second, shifted per unit. Unit `n` belongs to player `n - 1`.
fn intents_for(units: u32, tick: u64) -> Vec<Intent> {
    (1..=units as i64)
        .map(|id| {
            let quarter = (tick / game_sim::TICKS_PER_SECOND as u64 + id as u64) % 4;
            let (x, z) = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)][quarter as usize];
            Intent {
                player: PlayerId(id as u32 - 1),
                actor: SimId(id),
                action: Action::Move { x, z },
            }
        })
        .collect()
}

/// One row of the table.
#[derive(Debug, Clone, Copy)]
struct Row {
    units: u32,
    tick: Stats,
    capture: Stats,
    restore: Stats,
    hash: Stats,
    rollback: Stats,
}

/// How long `f` takes on the wall clock: the one place the benchmark reads a
/// clock, which the simulation itself never may (`clippy.toml`).
#[expect(
    clippy::disallowed_methods,
    reason = "a benchmark measures wall-clock time; nothing simulated reads it"
)]
fn timed(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

/// Per tick: run `ticks` ticks with intents, divide.
fn measure_tick(units: u32, ticks: u32) -> Duration {
    let mut app = headless_app(scenario(units));
    let total = timed(|| {
        for tick in 1..=u64::from(ticks) {
            let mut buffer = app.world_mut().resource_mut::<IntentBuffer>();
            for intent in intents_for(units, tick) {
                buffer.push(intent);
            }
            app.update();
        }
    });
    total / ticks
}

/// Per operation, over `reps` repetitions on a state that has run a second.
fn measure_snapshot_ops(units: u32, reps: u32) -> (Duration, Duration, Duration) {
    let mut app = headless_app(scenario(units));
    for tick in 1..=game_sim::TICKS_PER_SECOND as u64 {
        let mut buffer = app.world_mut().resource_mut::<IntentBuffer>();
        for intent in intents_for(units, tick) {
            buffer.push(intent);
        }
        app.update();
    }
    let world = app.world_mut();

    let capture = timed(|| {
        for _ in 0..reps {
            black_box(Snapshot::capture(world));
        }
    }) / reps;

    let snapshot = Snapshot::capture(world);
    let restore = timed(|| {
        for _ in 0..reps {
            snapshot.restore(world);
        }
    }) / reps;

    let hash = timed(|| {
        for _ in 0..reps {
            black_box(black_box(&snapshot).hash());
        }
    }) / reps;
    (capture, restore, hash)
}

/// One rollback: a late intent for the oldest tick in the window, so the
/// session restores and re-simulates `window` ticks.
fn measure_rollback(units: u32, window: usize) -> Duration {
    let mut session = RollbackSession::new(headless_app(scenario(units)), window);
    for tick in 1..=window as u64 {
        session.advance(intents_for(units, tick));
    }
    let oldest = SimTick(session.tick().0 + 1 - window as u64);
    let late = Intent {
        player: PlayerId(0),
        actor: SimId(1),
        action: Action::Idle,
    };
    let mut resimulated = 0;
    let elapsed = timed(|| {
        resimulated = session
            .insert_late(oldest, late)
            .expect("the oldest tick is inside the window");
    });
    assert_eq!(resimulated, window, "the rollback re-simulated the window");
    elapsed
}

fn measure(config: &Config, units: u32) -> Row {
    let reps = 100;
    let ops: Vec<_> = (0..config.runs)
        .map(|_| measure_snapshot_ops(units, reps))
        .collect();
    Row {
        units,
        tick: Stats::measure(config.runs, || measure_tick(units, config.ticks)),
        capture: Stats::of(ops.iter().map(|o| o.0).collect()),
        restore: Stats::of(ops.iter().map(|o| o.1).collect()),
        hash: Stats::of(ops.iter().map(|o| o.2).collect()),
        rollback: Stats::measure(config.runs, || measure_rollback(units, config.window)),
    }
}

/// A duration in microseconds, three significant places at most.
fn micros(d: Duration) -> String {
    let us = d.as_secs_f64() * 1e6;
    match us {
        us if us >= 100.0 => format!("{us:.0}"),
        us if us >= 10.0 => format!("{us:.1}"),
        us => format!("{us:.2}"),
    }
}

fn cell(stats: Stats) -> String {
    format!(
        "{} ({}–{})",
        micros(stats.median),
        micros(stats.min),
        micros(stats.max)
    )
}

/// How many ticks fit into one 60 Hz frame (16.7 ms) if a rollback must
/// restore once, then capture and simulate each re-simulated tick: the upper
/// bound of the rollback window on this machine, with nothing else to do in
/// the frame.
fn ticks_per_frame(row: &Row) -> u64 {
    let frame = Duration::from_secs(1) / 60;
    let per_tick = row.tick.median + row.capture.median;
    let left = frame.saturating_sub(row.restore.median);
    (left.as_nanos() / per_tick.as_nanos().max(1)) as u64
}

fn main() {
    let config = match Config::from_args(std::env::args().skip(1)) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("bench: {e}");
            eprintln!("usage: bench [--units 2,100,1000] [--ticks 600] [--runs 7] [--window 8]");
            std::process::exit(2);
        }
    };
    if cfg!(debug_assertions) {
        eprintln!("bench: debug build; numbers are only comparable with `--release`");
    }
    println!(
        "{} ticks per run, {} runs, rollback window {}; median (min–max) in µs",
        config.ticks, config.runs, config.window
    );
    println!();
    println!(
        "| Units | Tick | Ticks/s | Capture | Restore | Hash | Rollback of {} | Ticks per 60 Hz frame |",
        config.window
    );
    println!("| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for &units in &config.units {
        let row = measure(&config, units);
        println!(
            "| {} | {} | {:.0} | {} | {} | {} | {} | {} |",
            row.units,
            cell(row.tick),
            1.0 / row.tick.median.as_secs_f64(),
            cell(row.capture),
            cell(row.restore),
            cell(row.hash),
            cell(row.rollback),
            ticks_per_frame(&row),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn parses_flags_and_keeps_defaults() {
        assert_eq!(Config::from_args(args("")).unwrap(), Config::default());
        let config = Config::from_args(args("--units 5,50 --runs 3")).unwrap();
        assert_eq!(config.units, [5, 50]);
        assert_eq!(config.runs, 3);
        assert_eq!(config.ticks, Config::default().ticks);
        for bad in [
            "--units",
            "--runs 0",
            "--ticks x",
            "--units 1,,2",
            "--speed 2",
        ] {
            assert!(
                Config::from_args(args(bad)).is_err(),
                "`{bad}` was accepted"
            );
        }
    }

    #[test]
    fn median_and_spread() {
        let ms = Duration::from_millis;
        let odd = Stats::of(vec![ms(5), ms(1), ms(3)]);
        assert_eq!((odd.median, odd.min, odd.max), (ms(3), ms(1), ms(5)));
        assert_eq!(Stats::of(vec![ms(4), ms(2)]).median, ms(3));
    }

    #[test]
    fn every_unit_gets_a_valid_intent_each_tick() {
        let intents = intents_for(3, 1);
        assert_eq!(intents.len(), 3);
        for (n, intent) in intents.iter().enumerate() {
            assert_eq!(intent.actor, SimId(n as i64 + 1));
            assert_eq!(intent.player, PlayerId(n as u32));
            assert!(game_sim::game::validate(&intent.action));
        }
        assert_ne!(intents_for(1, 1)[0].action, intents_for(1, 61)[0].action);
    }

    /// A tiny run of every measurement, so the benchmark cannot rot unseen.
    #[test]
    fn a_small_run_measures_every_figure() {
        let config = Config {
            units: vec![3],
            ticks: 5,
            runs: 1,
            window: 4,
        };
        let row = measure(&config, 3);
        assert!(row.tick.median > Duration::ZERO);
        assert!(row.rollback.median > Duration::ZERO);
        assert!(ticks_per_frame(&row) > 0);
    }
}
