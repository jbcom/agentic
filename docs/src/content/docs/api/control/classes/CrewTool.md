---
editUrl: false
next: false
prev: false
title: "CrewTool"
---

Defined in: [packages/agentic-control/src/crews/crew-tool.ts:92](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/crew-tool.ts#L92)

Crew tool for invoking agentic-crew from TypeScript

## Examples

```typescript
const crewTool = new CrewTool();

// List available crews
const crews = await crewTool.listCrews();

// Run a crew
const result = await crewTool.invokeCrew({
  package: 'otterfall',
  crew: 'game_builder',
  input: 'Create a QuestComponent',
});
```

```typescript
import { tool } from 'ai';
import { z } from 'zod';

const crewTool = new CrewTool();

export const invokeCrewTool = tool({
  description: 'Delegate a task to a specialized AI crew',
  parameters: z.object({
    package: z.string(),
    crew: z.string(),
    input: z.string(),
  }),
  execute: async ({ package: pkg, crew, input }) => {
    const result = await crewTool.invokeCrew({ package: pkg, crew, input });
    if (!result.success) throw new Error(result.error);
    return result.output;
  },
});
```

## Constructors

### Constructor

> **new CrewTool**(`config?`): `CrewTool`

Defined in: [packages/agentic-control/src/crews/crew-tool.ts:95](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/crew-tool.ts#L95)

#### Parameters

##### config?

[`CrewToolConfig`](/api/control/interfaces/crewtoolconfig/)

#### Returns

`CrewTool`

## Methods

### getCrewInfo()

> **getCrewInfo**(`packageName`, `crewName`): `Promise`\<[`CrewInfo`](/api/control/interfaces/crewinfo/)\>

Defined in: [packages/agentic-control/src/crews/crew-tool.ts:136](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/crew-tool.ts#L136)

Get detailed information about a specific crew

#### Parameters

##### packageName

`string`

##### crewName

`string`

#### Returns

`Promise`\<[`CrewInfo`](/api/control/interfaces/crewinfo/)\>

***

### invokeCrew()

> **invokeCrew**(`options`): `Promise`\<[`CrewResult`](/api/control/interfaces/crewresult/)\>

Defined in: [packages/agentic-control/src/crews/crew-tool.ts:167](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/crew-tool.ts#L167)

Invoke a crew with the given input

#### Parameters

##### options

[`InvokeCrewOptions`](/api/control/interfaces/invokecrewoptions/)

#### Returns

`Promise`\<[`CrewResult`](/api/control/interfaces/crewresult/)\>

***

### listCrews()

> **listCrews**(): `Promise`\<[`CrewInfo`](/api/control/interfaces/crewinfo/)[]\>

Defined in: [packages/agentic-control/src/crews/crew-tool.ts:108](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/crew-tool.ts#L108)

List all available crews across all packages

#### Returns

`Promise`\<[`CrewInfo`](/api/control/interfaces/crewinfo/)[]\>
