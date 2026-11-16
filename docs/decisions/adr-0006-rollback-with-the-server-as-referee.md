---
title: "ADR-0006 — Rollback netcode with the server as referee"
---
# ADR-0006 — Rollback netcode with the server as referee

**Status:** accepted

**Context:** PvP should feel instant even under latency. Lockstep stalls everyone on the slowest input; client-server with server simulation means the server, not the player, decides what happened — and costs hosting per tick.

**Decision:** **Rollback netcode:** every client simulates the whole game locally at full frame rate and predicts; when a late intent arrives, it rewinds to the last confirmed tick and re-simulates to the present. A **server acts as referee**: it validates intents, owns the authoritative tick, decides conflicts, and sends each client only the observation it is allowed to see. When they disagree, the server wins.

**Rejected alternatives:** *Lockstep* (stalls on the slowest player) and *pure server simulation* (input lag for everyone; server load per tick).

**Consequences:** Local input has zero latency; the deterministic simulation and serialisable intents (ADR-0001, ADR-0002) make the rewind exact. Costs: the simulation must be fast enough to re-run several ticks per frame; the server is a trusted component; desyncs need detection and handling.
