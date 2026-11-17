---
title: "ADR-0012 — Genre-dependent decisions ship as template defaults, decided per game"
---
# ADR-0012 — Genre-dependent decisions ship as template defaults, decided per game

**Status:** accepted

**Context:** The template is used before anyone knows the genre: RPG, racing game, RTS, shooter, 2D or 3D. Some decisions hold for every genre (a deterministic tick simulation, intents, stable IDs, one RNG, snapshots). Others depend on the genre: how players connect, the tick rate and number type, the physics engine, the gameplay itself, how the client looks. Leaving those out makes the template untestable and the first session of every game start from nothing; fixing them as accepted makes the template quietly choose a genre.

**Decision:** A genre-dependent decision is recorded as an ADR with status **`default`**. It ships a basic, working, tested implementation, so the rest of the template can be built and tested against it, and a **To replace** section naming exactly the code and docs it owns. The first sessions of a new game decide every default: **accept** it (status becomes `accepted`), **replace** it (a new ADR supersedes it, the owned code is rewritten), or **drop** it (the owned code is deleted, the status becomes `superseded`). Nobody builds new code on a default without asking the maintainer. Defaults from this ADR on: ADR-0006 (rollback netcode), ADR-0013 (the example game), ADR-0014 (tick rate and number type), ADR-0015 (Avian; the rest of ADR-0011 stays accepted), ADR-0016 (the client shell), ADR-0017 (the save and replay file format).

**Rejected alternatives:** *No genre code at all* (nothing to test the core against; the client would show nothing), *accepted decisions for one genre* (the template would be a game, not a template), *feature flags per genre* (every combination would need testing and maintaining).

**Consequences:** The template runs, plays and is tested end to end, while a new game sees at a glance which decisions are still open. Costs: defaults are code that may be thrown away; the core must stay independent of them (the `game` module contract in `crates/game_sim/src/game/mod.rs`), or replacing one would ripple through everything.
