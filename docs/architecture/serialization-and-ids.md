---
title: "Serialization & IDs — what travels the network"
---
# Serialization & IDs — what travels the network

Two kinds of data ever leave the simulation: **intents** in, **observations** out. Both are plain `serde` data (ADR-0002); Bevy's `Reflect` is not used on the wire — enum discriminants and field order stay under our control.

Rules:

- Simulation entities are identified by **`SimId`** (ADR-0008), a dense `i64` assigned in spawn order. Never serialise `Entity`.
- `Action` variants and observation fields are **stable once released**: adding is fine, renumbering or changing meaning is a breaking change that needs a version flag.
- Serialisation is deterministic: `BTreeMap`, not `HashMap`; sorted lists, not iteration order.
- Everything the network accepts is untrusted input; see [[security|Security]].

How this feeds rollback: [[network|Network]].
