---
title: "Decisions — all ADRs with their status"
---
# Decisions — all ADRs with their status

One row per ADR; the next free number is **0018**.

| Status | Meaning |
| --- | --- |
| `accepted` | Holds for every game from this template. Re-opened only by a new ADR. |
| `default` | Genre-dependent: the template ships a working implementation, but each game decides it — accept, replace or drop, following the ADR's *To replace* section (ADR-0012). Ask before building new code on it. |
| `proposed` | Planning material; ask before building code on it. |
| `superseded` | Replaced by a later ADR, named in its status line. |

| ADR | Decision | Status |
| --- | --- | --- |
| [[adr-0001-tick-simulation-in-fixedupdate-instead-of-delta\|0001]] | Tick simulation in `FixedUpdate` instead of frame deltas | accepted |
| [[adr-0002-intent-as-serialisable-data-instead-of-raw-input\|0002]] | Intent as serialisable data instead of raw input | accepted |
| [[adr-0003-ticks-instead-of-seconds-for-all-timers\|0003]] | Ticks instead of seconds for all timers | accepted |
| [[adr-0004-one-seeded-rng-resource-instead-of-scattered-generators\|0004]] | One seeded RNG resource instead of scattered generators | accepted |
| [[adr-0005-simulation-as-its-own-crate\|0005]] | The simulation as its own crate | accepted |
| [[adr-0006-rollback-with-the-server-as-referee\|0006]] | Rollback netcode with the server as referee | **default** |
| [[adr-0007-observation-as-the-single-code-path-for-reading\|0007]] | The observation as the single code path for reading state | accepted |
| [[adr-0008-simid-instead-of-entity-index\|0008]] | `SimId` instead of entity index for ordering and serialisation | accepted |
| [[adr-0009-platform-neutrality\|0009]] | Platform neutrality: the simulation builds for wasm32 | accepted |
| [[adr-0010-central-balance-resource\|0010]] | Central balance resource instead of magic numbers | accepted |
| [[adr-0011-rust-and-bevy\|0011]] | Rust with Bevy (Avian: see 0015) | accepted |
| [[adr-0012-template-defaults-decided-per-game\|0012]] | Genre-dependent decisions ship as template defaults, decided per game | accepted |
| [[adr-0013-example-game-units-on-an-arena\|0013]] | The example game: units moving on an arena | **default** |
| [[adr-0014-sixty-ticks-per-second-with-f32\|0014]] | 60 ticks per second with f32 maths | **default** |
| [[adr-0015-avian-for-physics\|0015]] | Avian for physics | **default** |
| [[adr-0016-client-shell\|0016]] | The client shell: screens, input actions, localisation, debug overlay | **default** |
| [[adr-0017-postcard-files-with-a-versioned-header\|0017]] | Save and replay files: postcard behind a versioned header | **default** |

The `accepted` rows are the **solid decisions** of the template. The `default` rows are the first thing a new game decides.
