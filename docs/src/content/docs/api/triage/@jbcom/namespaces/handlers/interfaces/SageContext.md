---
editUrl: false
next: false
prev: false
title: "SageContext"
---

Defined in: [packages/triage/src/handlers/sage.ts:19](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L19)

Context that can be provided to Sage for better responses

## Properties

### currentContext?

> `optional` **currentContext**: `string`

Defined in: [packages/triage/src/handlers/sage.ts:27](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L27)

Current working directory or file context

***

### issueContext?

> `optional` **issueContext**: `string`

Defined in: [packages/triage/src/handlers/sage.ts:25](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L25)

Issue or PR context if available

***

### keyFiles?

> `optional` **keyFiles**: `Record`\<`string`, `string`\>

Defined in: [packages/triage/src/handlers/sage.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L23)

Content from key files (README, CLAUDE.md, etc.)

***

### repoStructure?

> `optional` **repoStructure**: `string`

Defined in: [packages/triage/src/handlers/sage.ts:21](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L21)

Repository structure (file list)
