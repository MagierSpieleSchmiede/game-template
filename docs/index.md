---
title: "Game Template"
description: A template for deterministic Bevy games in Rust — specification, architecture, decisions and open problems, published as the project's vault.
---
# Game Template

**Game Template** is the starting point for every new game of Magier SpieleSchmiede: a game in **Rust** with **Bevy**, whose entire gameplay is a **deterministic, headless tick simulation** in its own crate; the presentation is a pure observer that writes back exactly one thing, the **intent**. Randomness comes from the **one seeded RNG resource**; all durations are ticks. Planned for Windows, Linux, macOS, Android and iOS, with keyboard/mouse and gamepad alike. Genre-dependent choices — how players connect, tick rate, physics, the gameplay itself, the client's look — ship as **template defaults**: working code a new game accepts, replaces or drops ([[decisions|Decisions]]). Later come PvP, AI opponents, add-ons and more — designed for, not built yet. This vault documents the template: specification, architecture, decisions, test strategy and open problems, one page per topic.

> [!info] Status
> Template, not a game. What was last done and what comes next: **[[status|Status]]**. The next tasks: **[[backlog|Backlog]]**.

## Start here

1. [[status|Status]] — the handover: what was last done, what comes next
2. [[overview|Overview]] — the system in one diagram
3. [[project-structure|Project structure]] — the crate and plugin layout, and why each part exists
4. [[simulation-model|Simulation model]] — the one big bet: a deterministic tick chain
5. [[backlog|Backlog]] — the next tasks, with dependencies
6. [[roadmap|Roadmap & milestones]] and [[extension-points|Extension points]] — what comes after the prototype
7. [[decisions|Decisions]] — all ADRs with their status
8. [[security|Security]] — how the server and PvP are kept safe

## Areas

| Area | Pages |
| --- | --- |
| Architecture | [[overview\|Overview]] · [[project-structure\|Project structure]] · [[simulation-model\|Simulation model]] · [[intent-and-controls\|Intent & controls]] · [[determinism\|Determinism]] · [[serialization-and-ids\|Serialization & IDs]] · [[observation\|Observation]] · [[platforms\|Platforms]] |
| Later | [[network\|Network]] · [[security\|Security]] |
| Decisions | [[decisions\|Decisions]] — all ADRs with their status |
| Specification | [[spec-001-prototype\|Spec 001 — fundamental prototype]] |
| Testing | [[test-strategy\|Test strategy]] · [[headless-integration-test\|Headless integration test]] · [[determinism-test\|Determinism test]] · [[benchmarks\|Benchmarks]] |
| Planning | [[status\|Status]] · [[backlog\|Backlog]] · [[roadmap\|Roadmap & milestones]] · [[extension-points\|Extension points]] |
| Problems | [[problem-log\|Problem log]] |
| Conventions | [[conventions\|Conventions]] · [[adr-nnnn\|ADR template]] · [[p-nnn\|P template]] |
| Glossary | [[glossary\|Glossary]] — terms from intent to rollback |

## Using this template for a new game

1. Create the repository from this template and rename `game_sim` / `game_client` to your game.
2. Rewrite [[spec-001-prototype|Spec 001]], [[index]] and [[status|Status]] for your game; keep the structure and the conventions.
3. Decide every **template default** (the `default` rows in [[decisions|Decisions]]): accept it, replace it, or drop it following its *To replace* section. The `accepted` ADRs are the solid decisions; supersede them only with a new ADR.
4. Point `site/quartz.config.ts` and the deploy workflow at your repository.
