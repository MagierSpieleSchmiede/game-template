---
title: "ADR-0017 — Save and replay files: postcard behind a versioned header"
---
# ADR-0017 — Save and replay files: postcard behind a versioned header

**Status:** default (ADR-0012)

**Context:** Save games and replays must become bytes: to store them, to attach a replay to a bug report, to commit a regression replay that CI plays back. The encoding must build for every target (the simulation has no file system on wasm), reproduce `f32` values bit for bit, and stay stable. A deterministic replay only reproduces its run on the simulation behaviour that recorded it, whatever the format, so old files must be recognised and refused rather than silently diverge.

**Decision:** **postcard** (no_std, stable wire format since 1.0, floats as raw bits) behind a header: magic `GSIM`, the container `FORMAT_VERSION`, the file kind (save or replay), and the game's `SIM_VERSION`. `game_sim::file` encodes and decodes; loading refuses foreign bytes, other formats, the wrong kind, other simulation versions, and damaged or padded payloads, each with its own `FileError`. A save also stores its state hash and is refused if the state does not match it. Replays record intents as they arrived, before the checks, so playback runs validation and control again. The simulation only produces bytes; the client decides where they are stored. A regression replay (`crates/game_sim/tests/data/regression.replay`) is played back by CI on every platform.

**Rejected alternatives:** *RON or JSON* (readable and diffable, but 5–10× larger and slower; readability matters little for files that must be re-recorded on every behaviour change anyway), *MessagePack/CBOR* (self-describing, but a deterministic replay cannot survive schema changes by tolerance alone; bigger than postcard), *bincode* (a second major API in a short time; no specified wire format).

**Consequences:** Saves and replays are small (ten seconds of five players: about 1.2 KB) and identical on every platform; the same encoding can later carry intents over the network. Costs: the files are not human-readable (`Debug` printing is the way to inspect them); every behaviour change means bumping `SIM_VERSION` and re-recording the regression replay, which the failing test explains; old saves stop loading after such a bump, so a shipped game needs a migration path before it promises save compatibility.

**To replace:** change `crates/game_sim/src/file.rs` (and the `postcard` dependency in both `Cargo.toml`s), bump `FORMAT_VERSION`, re-record the regression replay. `save.rs` and `replay.rs` only call `file::encode`/`file::decode`.
