---
editUrl: false
next: false
prev: false
title: "triageAnalysisSchema"
---

> `const` **triageAnalysisSchema**: `ZodObject`\<\{ `codeReview`: `ZodOptional`\<`ZodObject`\<\{ `comments`: `ZodArray`\<`ZodObject`\<\{ `content`: `ZodString`; `file`: `ZodString`; `line`: `ZodOptional`\<`ZodNumber`\>; `severity`: `ZodOptional`\<`ZodEnum`\<\{ `high`: ...; `low`: ...; `medium`: ...; \}\>\>; `type`: `ZodEnum`\<\{ `issue`: `"issue"`; `praise`: `"praise"`; `question`: `"question"`; `suggestion`: `"suggestion"`; \}\>; \}, `$strip`\>\>; `impact`: `ZodEnum`\<\{ `critical`: `"critical"`; `high`: `"high"`; `low`: `"low"`; `medium`: `"medium"`; \}\>; `status`: `ZodEnum`\<\{ `approve`: `"approve"`; `comment`: `"comment"`; `request_changes`: `"request_changes"`; \}\>; `suggestedLabels`: `ZodArray`\<`ZodString`\>; `summary`: `ZodString`; \}, `$strip`\>\>; `issueAnalysis`: `ZodOptional`\<`ZodObject`\<\{ `actionItems`: `ZodArray`\<`ZodString`\>; `estimate`: `ZodOptional`\<`ZodNumber`\>; `labels`: `ZodArray`\<`ZodString`\>; `priority`: `ZodEnum`\<\{ `backlog`: `"backlog"`; `critical`: `"critical"`; `high`: `"high"`; `low`: `"low"`; `medium`: `"medium"`; \}\>; `summary`: `ZodString`; `title`: `ZodString`; `type`: `ZodEnum`\<\{ `bug`: `"bug"`; `chore`: `"chore"`; `docs`: `"docs"`; `epic`: `"epic"`; `feature`: `"feature"`; `task`: `"task"`; \}\>; \}, `$strip`\>\>; `triage`: `ZodString`; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/triage.ts:5](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/triage.ts#L5)
