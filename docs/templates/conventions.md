---
title: "Conventions — how pages, ADRs and P-entries are written"
---
# Conventions — how pages, ADRs and P-entries are written

## Pages

- **Title:** "Topic — the point in half a sentence" (front matter `title:` and the H1 say the same).
- **One source per fact:** never copy what another page already says; link to it. Which fact lives where:

| Fact | Lives in |
| --- | --- |
| Why a decision was made | the ADR |
| The invariants of the simulation | [[simulation-model\|Simulation model]] |
| What a task is and needs | [[backlog\|Backlog]] |
| What was last done, what comes next | [[status\|Status]] |
| Open questions | [[problem-log\|Problem log]] |
| Terms | [[glossary\|Glossary]] |

- **References** as Obsidian links: `[[filename|display text]]` (filename without `.md`).
- **Callouts** for the four standard boxes: `> [!info]` status, `> [!tip]` how to start, `> [!important]` what a page is for, `> [!warning]` what can break.
- The vault is in **English**; identifiers in code are English too.

## ADRs

One decision per ADR, following [[adr-nnnn|ADR template]], with a row in [[decisions|Decisions]]. Status is `proposed` or `accepted`; an accepted ADR is only superseded by a new ADR.

## P-entries

One open question per page, following [[p-nnn|P template]], with a row in the [[problem-log|Problem log]] that names the next step.

## Status

Whoever finishes a task **overwrites** [[status|Status]] (never appends): last done, next task with its *Read for that*, hints for the next session. About 30 lines at most.
