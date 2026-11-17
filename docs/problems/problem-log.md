---
title: "Problem log — open questions and their next steps"
---
# Problem log — open questions and their next steps

One row per problem; the next free number is **P-002**. A problem gets a page (`problems/p-nnn-<slug>.md`, template: [[p-nnn|P template]]) and a row here, with its next step.

| P | Problem | Next step | Milestone |
| --- | --- | --- | --- |
| [[p-001-intent-arbitration-and-tick-addressing\|P-001]] | Networked intents need a tick address and an arbitration rule | decide with the game's network model (ADR-0006 or its replacement) | when a game goes networked |

Typical first problems for a new game from this template:

- Cross-platform floating-point determinism for real PvP (research; possibly fixed-point math).
- Transport crate and protocol versioning for the network.
- Which physics queries are deterministic enough for the simulation.
