---
title: "Security — how the server, PvP and add-ons are kept safe"
---
# Security — how the server, PvP and add-ons are kept safe

Mandatory reading for every change that touches the network, external input or add-ons.

- **The client is in the hands of the enemy.** Every intent that reaches a server or another client is untrusted input: validate range, rate and ownership before it enters a simulation.
- **The server is the referee** (ADR-0006): it validates intents, owns the authoritative tick and decides conflicts. A client never trusts another client's state, only the server's observation.
- **Visibility is enforced by the server** ([[observation|Observation]]): what a team cannot see, it never receives. A client cannot leak what it never had.
- **Add-ons produce intents, nothing else** (see [[extension-points|Extension points]]): they run sandboxed, read through the observation, and can never touch simulation state, the network or the file system.
- **No secrets in the client** and none in the repository (`.env`, `.secrets` are gitignored and never committed).
- **Determinism is integrity:** a desync between client and server is a bug and a cheat detector at once; the determinism test is part of the security posture.
