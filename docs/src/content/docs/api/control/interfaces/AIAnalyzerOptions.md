---
editUrl: false
next: false
prev: false
title: "AIAnalyzerOptions"
---

Defined in: [packages/agentic-control/src/triage/analyzer.ts:148](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L148)

## Extends

- [`ProviderOptions`](/api/control/interfaces/provideroptions/)

## Properties

### apiKey?

> `optional` **apiKey**: `string`

Defined in: [packages/agentic-control/src/core/providers.ts:243](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/providers.ts#L243)

API key (defaults to provider-specific env var)

#### Inherited from

[`ProviderOptions`](/api/control/interfaces/provideroptions/).[`apiKey`](/api/control/interfaces/provideroptions/#apikey)

***

### model?

> `optional` **model**: `string`

Defined in: [packages/agentic-control/src/core/providers.ts:241](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/providers.ts#L241)

Model to use

#### Inherited from

[`ProviderOptions`](/api/control/interfaces/provideroptions/).[`model`](/api/control/interfaces/provideroptions/#model)

***

### provider?

> `optional` **provider**: `string`

Defined in: [packages/agentic-control/src/core/providers.ts:239](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/providers.ts#L239)

AI provider: anthropic, openai, google, mistral, azure, ollama

#### Inherited from

[`ProviderOptions`](/api/control/interfaces/provideroptions/).[`provider`](/api/control/interfaces/provideroptions/#provider)

***

### repo?

> `optional` **repo**: `string`

Defined in: [packages/agentic-control/src/triage/analyzer.ts:150](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L150)

Repository for GitHub operations (required for issue creation)
