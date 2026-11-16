---
title: "ADR-0007 — The observation as the single code path for reading state"
---
# ADR-0007 — The observation as the single code path for reading state

**Status:** accepted

**Context:** HUD, AI, add-ons and the network all need simulation state. If each reads the ECS world directly, visibility is enforced in many places (or nowhere), AI sees more than players, and the network leaks data.

**Decision:** One observation interface: a function that snapshots what a given team may see this tick, with a visibility filter. HUD, AI, add-ons and network sending read it and nothing else. It returns plain, serialisable data.

**Rejected alternative:** Direct `Query` access per consumer. More flexible, but visibility, versioning and auditing become impossible.

**Consequences:** Visibility has one place to be audited; AI trains on exactly what a player sees; the network sends exactly what the observation returns. Costs: one more layer, and the observation format must be versioned like the intent.
