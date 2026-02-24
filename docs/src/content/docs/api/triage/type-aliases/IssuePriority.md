---
editUrl: false
next: false
prev: false
title: "IssuePriority"
---

> **IssuePriority** = `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`

Defined in: [packages/triage/src/providers/types.ts:19](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L19)

Triage Provider Abstraction

Defines common interfaces for issue tracking providers.
Allows agentic-triage to work with multiple backends:
- GitHub Issues
- Beads (local-first, AI-native)
- Jira (enterprise)
- Linear (modern)

Similar to how AI providers work,
agentic-triage has multiple triage/issue providers.
