---
title: "T-007 — Keep the determinism tests green under every change"
---
# T-007 — Keep the determinism tests green under every change

**Status:** open · **Scope:** game · **Depends on:** all

**Read for that:** [[determinism-test|Determinism test]]

**What:** A standing task: the determinism test and the regression replay pass after every change; their committed values change only on deliberate behaviour changes, with `SIM_VERSION` bumped.

**Done when:** Never closed; checked by CI on every push.
