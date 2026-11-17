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
| What a task is and needs | its page in `backlog/`, listed in the [[backlog\|Backlog]] |
| What was last done, what comes next | [[status\|Status]] |
| Open questions | [[problem-log\|Problem log]] |
| Terms | [[glossary\|Glossary]] |

- **References** as Obsidian links: `[[filename|display text]]` (filename without `.md`).
- **Callouts** for the four standard boxes: `> [!info]` status, `> [!tip]` how to start, `> [!important]` what a page is for, `> [!warning]` what can break.
- The vault is in **English**; identifiers in code are English too.

## ADRs

One decision per ADR, following [[adr-nnnn|ADR template]], with a row in [[decisions|Decisions]]. Status is `proposed`, `accepted`, `default` or `superseded` (meanings in [[decisions|Decisions]]); an accepted ADR is only superseded by a new ADR. A `default` ADR adds a **To replace** section naming the code and docs it owns (ADR-0012).

## Tasks

One task per page in `backlog/`, following [[t-nnn|Task template]], with a row in the [[backlog|Backlog]]. Scope `game` (done once per game made from the template) or `template` (improves the template). Status `open`, `in progress` or `done`, kept the same on the page and in the row.

## P-entries

One open question per page, following [[p-nnn|P template]], with a row in the [[problem-log|Problem log]] that names the next step.

## Status

Whoever finishes a task **overwrites** [[status|Status]] (never appends): last done, next task with its *Read for that*, hints for the next session. About 30 lines at most.
