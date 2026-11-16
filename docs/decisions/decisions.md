---
title: "Decisions — all ADRs with their status"
---
# Decisions — all ADRs with their status

One row per ADR; the next free number is **0012**. An accepted ADR is not re-opened without a new ADR. A proposed ADR is planning material; ask before building code on it.

| ADR | Decision | Status |
| --- | --- | --- |
| [[adr-0001-tick-simulation-in-fixedupdate-instead-of-delta\|0001]] | Tick simulation in `FixedUpdate` instead of frame deltas | accepted |
| [[adr-0002-intent-as-serialisable-data-instead-of-raw-input\|0002]] | Intent as serialisable data instead of raw input | accepted |
| [[adr-0003-ticks-instead-of-seconds-for-all-timers\|0003]] | Ticks instead of seconds for all timers | accepted |
| [[adr-0004-one-seeded-rng-resource-instead-of-scattered-generators\|0004]] | One seeded RNG resource instead of scattered generators | accepted |
| [[adr-0005-simulation-as-its-own-crate\|0005]] | The simulation as its own crate | accepted |
| [[adr-0006-rollback-with-the-server-as-referee\|0006]] | Rollback netcode with the server as referee | accepted |
| [[adr-0007-observation-as-the-single-code-path-for-reading\|0007]] | The observation as the single code path for reading state | accepted |
| [[adr-0008-simid-instead-of-entity-index\|0008]] | `SimId` instead of entity index for ordering and serialisation | accepted |
| [[adr-0009-platform-neutrality\|0009]] | Platform neutrality: the simulation builds for wasm32 | accepted |
| [[adr-0010-central-balance-resource\|0010]] | Central balance resource instead of magic numbers | accepted |
| [[adr-0011-rust-and-bevy\|0011]] | Rust with Bevy and Avian | accepted |

These are the **solid decisions** of the template. A new game deletes rows only with a new ADR that supersedes them.
