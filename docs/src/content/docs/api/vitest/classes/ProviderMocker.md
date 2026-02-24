---
editUrl: false
next: false
prev: false
title: "ProviderMocker"
---

Defined in: [providers.ts:140](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L140)

AI Provider mocking utilities class.

Provides methods for mocking AI provider responses during testing.

## Example

```typescript
import { ProviderMocker } from 'vitest-agentic-control';

const mocker = new ProviderMocker();

// Mock Anthropic provider
mocker.mockAnthropic({
  response: 'Hello! I am Claude.',
  usage: { promptTokens: 10, completionTokens: 20, totalTokens: 30 },
});

// Mock OpenAI provider with streaming
mocker.mockOpenAI({
  response: 'Hello! I am GPT.',
  stream: true,
});

// Mock a specific model
const model = mocker.createMockModel('anthropic', 'claude-sonnet-4-20250514');
```

## Constructors

### Constructor

> **new ProviderMocker**(`options?`): `ProviderMocker`

Defined in: [providers.ts:153](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L153)

#### Parameters

##### options?

[`ProviderMockerOptions`](/api/vitest/interfaces/providermockeroptions/) = `{}`

#### Returns

`ProviderMocker`

## Properties

### models

> `readonly` **models**: `Map`\<`string`, `MockModel`\>

Defined in: [providers.ts:151](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L151)

Track mock models

## Methods

### createMockModel()

> **createMockModel**(`provider`, `modelId`, `config?`): `MockModel`

Defined in: [providers.ts:330](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L330)

Create a mock model for a provider.

#### Parameters

##### provider

The provider type

`"anthropic"` | `"openai"` | `"google"` | `"mistral"` | `"azure"` | `"ollama"`

##### modelId

`string`

The model ID

##### config?

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) = `{}`

Response configuration

#### Returns

`MockModel`

Mock model instance

***

### getProviderConfig()

> **getProviderConfig**(`provider`): [`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) \| `undefined`

Defined in: [providers.ts:446](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L446)

Get the configuration for a provider.

#### Parameters

##### provider

The provider

`"anthropic"` | `"openai"` | `"google"` | `"mistral"` | `"azure"` | `"ollama"`

#### Returns

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) \| `undefined`

The configuration or undefined

***

### mockAiSdk()

> **mockAiSdk**(): `void`

Defined in: [providers.ts:392](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L392)

Mock the core AI SDK module.

#### Returns

`void`

***

### mockAllModules()

> **mockAllModules**(): `Record`\<`string`, `unknown`\>

Defined in: [providers.ts:414](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L414)

Mock all provider modules.

#### Returns

`Record`\<`string`, `unknown`\>

Dictionary of mocked modules

***

### mockAnthropic()

> **mockAnthropic**(`config?`): `void`

Defined in: [providers.ts:168](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L168)

Mock the Anthropic provider.

#### Parameters

##### config?

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) = `{}`

Response configuration

#### Returns

`void`

***

### mockAzure()

> **mockAzure**(`config?`): `void`

Defined in: [providers.ts:208](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L208)

Mock the Azure provider.

#### Parameters

##### config?

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) = `{}`

Response configuration

#### Returns

`void`

***

### mockGoogle()

> **mockGoogle**(`config?`): `void`

Defined in: [providers.ts:188](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L188)

Mock the Google provider.

#### Parameters

##### config?

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) = `{}`

Response configuration

#### Returns

`void`

***

### mockMistral()

> **mockMistral**(`config?`): `void`

Defined in: [providers.ts:198](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L198)

Mock the Mistral provider.

#### Parameters

##### config?

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) = `{}`

Response configuration

#### Returns

`void`

***

### mockOllama()

> **mockOllama**(`config?`): `void`

Defined in: [providers.ts:218](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L218)

Mock the Ollama provider.

#### Parameters

##### config?

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) = `{}`

Response configuration

#### Returns

`void`

***

### mockOpenAI()

> **mockOpenAI**(`config?`): `void`

Defined in: [providers.ts:178](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L178)

Mock the OpenAI provider.

#### Parameters

##### config?

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/) = `{}`

Response configuration

#### Returns

`void`

***

### resetAll()

> **resetAll**(): `void`

Defined in: [providers.ts:462](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L462)

Reset all mocks.

#### Returns

`void`

***

### restoreAll()

> **restoreAll**(): `void`

Defined in: [providers.ts:453](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L453)

Restore all mocked modules.

#### Returns

`void`

***

### setProviderResponse()

> **setProviderResponse**(`provider`, `config`): `void`

Defined in: [providers.ts:436](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L436)

Set a response for a specific provider.

#### Parameters

##### provider

The provider

`"anthropic"` | `"openai"` | `"google"` | `"mistral"` | `"azure"` | `"ollama"`

##### config

[`MockProviderResponse`](/api/vitest/interfaces/mockproviderresponse/)

Response configuration

#### Returns

`void`
