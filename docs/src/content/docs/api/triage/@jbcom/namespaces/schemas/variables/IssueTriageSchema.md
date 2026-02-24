---
editUrl: false
next: false
prev: false
title: "IssueTriageSchema"
---

> `const` **IssueTriageSchema**: `ZodObject`\<\{ `actionItems`: `ZodArray`\<`ZodString`\>; `estimate`: `ZodOptional`\<`ZodNumber`\>; `labels`: `ZodArray`\<`ZodString`\>; `priority`: `ZodEnum`\<\{ `backlog`: `"backlog"`; `critical`: `"critical"`; `high`: `"high"`; `low`: `"low"`; `medium`: `"medium"`; \}\>; `summary`: `ZodString`; `title`: `ZodString`; `type`: `ZodEnum`\<\{ `bug`: `"bug"`; `chore`: `"chore"`; `docs`: `"docs"`; `epic`: `"epic"`; `feature`: `"feature"`; `task`: `"task"`; \}\>; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/issue.ts:7](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/issue.ts#L7)
