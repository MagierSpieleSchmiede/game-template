---
title: "Extension points — what the architecture is designed for"
---
# Extension points — what the architecture is designed for

The template's architecture is a bet on a few later features. Design for them, do not build them early:

| Extension | Why the architecture fits | Where it will hook in |
| --- | --- | --- |
| **AI opponents / RL** | The simulation runs headless and deterministically; an agent acts exactly like a player by producing intents | `Intent`, [[headless-integration-test\|headless app]] |
| **PvP with rollback** | Deterministic ticks, serialisable intents, stable IDs | [[network\|Network]], ADR-0006 |
| **Add-ons** | Add-ons produce intents and read through the observation, sandboxed, never touching state | [[observation\|Observation]], [[security\|Security]] |
| **Spectators / replays** | A run is fully described by scenario + intent log | `Scenario`, `IntentBuffer` |
| **Server authority** | The server runs the same headless simulation and referees | `headless`, ADR-0006 |
| **Large worlds** | The observation filter already limits what a client receives | [[observation\|Observation]] |

Rule: no abstraction that does not serve this list or is used at least twice (see [[spec-001-prototype|Spec 001]]).
