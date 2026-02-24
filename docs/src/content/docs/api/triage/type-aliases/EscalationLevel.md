---
editUrl: false
next: false
prev: false
title: "EscalationLevel"
---

> **EscalationLevel** = `0` \| `1` \| `2` \| `3` \| `4` \| `5` \| `6`

Defined in: [packages/triage/src/escalation/state.ts:18](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L18)

The 7 escalation levels
- 0: Static Analysis (lint/tsc) - Free, instant
- 1: Complexity Evaluation (Ollama) - Free, routes to 2 or 3
- 2: Ollama Fix - Free, simple fixes
- 3: Jules Session - Free tier, complex work
- 4: Jules + Boosted Context - Free tier, more context
- 5: Human Review Queue - Free, awaits approval
- 6: Cloud Agent (Cursor) - Expensive, requires approval
