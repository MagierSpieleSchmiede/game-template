---
title: "ADR-0008 — SimId instead of entity index for ordering and serialisation"
---
# ADR-0008 — SimId instead of entity index for ordering and serialisation

**Status:** accepted

**Context:** Bevy `Entity` indices are ephemeral and unordered: the same scenario can assign different indices on different runs, and iteration order over archetypes is not a contract. Anything that orders gameplay, hashes on entities, or serialises them will desync.

**Decision:** Every simulated entity carries a stable `SimId` (`i64`, assigned in spawn order). All ordering, hashing and serialisation in the simulation keys on `SimId`, never on `Entity`.

**Rejected alternative:** Ordering by `Entity`. Free, but non-deterministic across runs and meaningless over the network.

**Consequences:** Deterministic ordering and stable wire formats; rollback snapshots reference entities that still mean the same thing after a rewind. Costs: one spawn counter to maintain and a lookup from ID to entity.
