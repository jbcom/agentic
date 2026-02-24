---
editUrl: false
next: false
prev: false
title: "createOllamaAgent"
---

> **createOllamaAgent**(`id`, `config?`, `options?`): `AgentDefinition`\<`string`\>

Defined in: [ollama.ts:70](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/ollama.ts#L70)

Create an Ollama-based agent for the registry

## Parameters

### id

`string`

### config?

[`OllamaConfig`](/api/providers/interfaces/ollamaconfig/) = `{}`

### options?

#### capabilities?

`Partial`\<`AgentCapabilities`\>

#### cost?

`number`

#### name?

`string`

#### priority?

`number`

## Returns

`AgentDefinition`\<`string`\>
