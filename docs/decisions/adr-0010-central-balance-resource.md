---
title: "ADR-0010 — Central balance resource instead of magic numbers"
---
# ADR-0010 — Central balance resource instead of magic numbers

**Status:** accepted

**Context:** Gameplay constants (speeds, damages, cooldowns) change constantly during balancing. Buried as magic numbers in systems, they are unfindable, untestable and undocumented — and later not exchangeable per game mode or by the server.

**Decision:** Every gameplay constant lives in the central `Balance` resource (one place, documented, typed). Systems read from it; magic numbers in systems are a review error. Later, balance variants become data.

**Rejected alternative:** Constants in the systems that use them. Local and obvious at first, but balancing becomes archaeology.

**Consequences:** Balancing happens in one file; constants can be serialised, diffed and shared with the server. Costs: one more parameter in almost every system signature.
