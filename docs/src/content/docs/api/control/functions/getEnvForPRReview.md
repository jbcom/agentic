---
editUrl: false
next: false
prev: false
title: "getEnvForPRReview"
---

> **getEnvForPRReview**(): `Record`\<`string`, `string`\>

Defined in: [packages/agentic-control/src/core/tokens.ts:390](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/tokens.ts#L390)

Create environment variables for PR review operations
Uses the configured PR review identity

## Returns

`Record`\<`string`, `string`\>

Object with GH_TOKEN and GITHUB_TOKEN set for PR review
