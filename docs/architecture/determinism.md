---
title: "Determinism — same seed, same inputs, same end state"
---
# Determinism — same seed, same inputs, same end state

The contract: **same start state, same inputs, same seed → same end state, byte for byte.** It is what makes the game testable without a window, replayable, and the foundation for [[network|rollback netcode]] and RL training.

What exactly is guaranteed, from weakest to strongest:

| Level | Meaning | Status in the template |
| --- | --- | --- |
| **Repeatability** | the same executable, same inputs → same end state | guaranteed; pinned by the [[determinism-test\|determinism test]] |
| **Cross-platform** | different supported platforms and builds → same end state | holds for the example on the CI platforms (the regression replay), as long as the floating-point discipline below is kept; not proven for every f32 computation or compiler setting |
| **Network** | all participants of a session reach the same state under the sync protocol | depends on the network model a game picks (ADR-0006); rollback and lockstep need cross-platform determinism, a server-authoritative model with corrections only needs repeatability on the server |

Each game states which level it needs in its spec; a game that needs cross-platform determinism for real PvP has the open floating-point question in the [[problem-log|Problem log]] to settle.

What it takes:

- **Ticks, not seconds** (ADR-0003): all durations are integer ticks; `TICKS_PER_SECOND` only converts per-second balance rates.
- **One seeded RNG** (ADR-0004): `SimRng`, Xoshiro256\*\* with SplitMix64, seeded from the scenario. No other randomness — enforced by `clippy.toml`.
- **No hash maps** in the simulation: `HashMap`/`HashSet` (std's, Bevy's, `hashbrown`'s) iterate in an order that depends on hashing or insertion history; `BTreeMap`/`BTreeSet` instead (enforced by `clippy.toml`, which also forbids `rand` and `getrandom` should they ever become dependencies).
- **Ordering by `SimId`** (ADR-0008): never by `Entity`.
- **Floating point discipline:** the same operations in the same order on all platforms; only operations IEEE 754 defines exactly (`+ - * /`, `sqrt`), no `sin`, `cos`, `powf` and the like, whose results come from the platform's maths library; cross-platform floating-point determinism for real PvP is an open question (see the problem log).
- **No clocks:** no `Instant::now`, no `SystemTime::now` in the simulation; count ticks.
- **One tick rate:** `SimPlugin` sets the fixed timestep to `TICKS_PER_SECOND` (Bevy's default is 64 Hz; the value 60 is a template default, ADR-0014); systems use `TICK_SECONDS`, never `Time`.
- **One seeding point:** `scenario::start_run` seeds `SimRng` from the scenario before the game spawns, for the client and the headless app alike; IDs come from the `SimIdCounter` resource.
- **Intents in actor order:** `TickIntents` is sorted by `SimId`, not by arrival.
- **Complete snapshots:** everything that changes during a run is captured by `Snapshot` (tick, RNG, ID counter, who controls which actor, the game's state, including every run-mutable game resource, as the contract in `game/mod.rs` requires), and the state hash covers all of it, including all four words of the RNG state; `tests/snapshot.rs` checks it by re-running 40 ticks from a restored snapshot. It **catches** forgotten components and resources that feed back into hashed state within those 40 ticks (pinned for a resource by a test-only countdown). It **cannot catch** forgotten state that only the presentation reads, or that feeds back later than 40 ticks; that is up to review, and to a test with a scenario that exercises it.

How it is tested: [[determinism-test|Determinism test]], and a committed regression replay that every CI platform plays back. What it enables: [[network|Network]].
