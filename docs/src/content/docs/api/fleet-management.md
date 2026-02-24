---
title: Fleet API Reference
description: Complete API reference for @agentic-dev-library/control Fleet management
---

# Fleet API Reference

Complete reference for the `Fleet` class from `@agentic-dev-library/control`.

## Installation

```bash
npm install @agentic-dev-library/control
```

## Import

```typescript
import { Fleet } from '@agentic-dev-library/control';
```

## Constructor

```typescript
const fleet = new Fleet(options?: FleetOptions);
```

### FleetOptions

| Option | Type | Description |
|--------|------|-------------|
| `apiKey` | `string` | Cursor API key (defaults to `CURSOR_API_KEY` env var) |
| `baseUrl` | `string` | API base URL (rarely needed) |

## Methods

### isApiAvailable()

Check if the Cursor API is available.

```typescript
isApiAvailable(): boolean
```

**Returns:** `true` if `CURSOR_API_KEY` is set, `false` otherwise.

**Example:**
```typescript
if (!fleet.isApiAvailable()) {
  console.error('Set CURSOR_API_KEY environment variable');
  process.exit(1);
}
```

---

### list()

List all agents.

```typescript
list(): Promise<Result<Agent[]>>
```

**Returns:** `Result<Agent[]>` - All agents in the fleet.

**Example:**
```typescript
const result = await fleet.list();
if (result.success && result.data) {
  for (const agent of result.data) {
    console.log(`${agent.id}: ${agent.status}`);
  }
}
```

---

### running()

List only running agents.

```typescript
running(): Promise<Result<Agent[]>>
```

**Returns:** `Result<Agent[]>` - Only agents with status `RUNNING`.

**Example:**
```typescript
const result = await fleet.running();
if (result.success && result.data) {
  console.log(`${result.data.length} agents currently running`);
}
```

---

### summary()

Get fleet summary statistics.

```typescript
summary(): Promise<Result<FleetSummary>>
```

**Returns:** `Result<FleetSummary>`

```typescript
interface FleetSummary {
  total: number;
  running: number;
  completed: number;
  failed: number;
  agents: Agent[];
}
```

**Example:**
```typescript
const result = await fleet.summary();
if (result.success && result.data) {
  const { total, running, completed, failed } = result.data;
  console.log(`Fleet: ${total} total, ${running} running`);
}
```

---

### spawn()

Spawn a new agent.

```typescript
spawn(options: SpawnOptions): Promise<Result<Agent>>
```

**Parameters:**

```typescript
interface SpawnOptions {
  repository: string;  // Full GitHub URL
  task: string;        // Task description
  ref?: string;        // Branch/tag/commit (default: 'main')
  target?: {
    autoCreatePr?: boolean;     // Create PR on completion
    branchName?: string;        // Custom branch name
    openAsCursorGithubApp?: boolean;  // Use Cursor app identity
  };
}
```

**Returns:** `Result<Agent>` - The spawned agent.

**Example:**
```typescript
const result = await fleet.spawn({
  repository: 'https://github.com/my-org/my-repo',
  task: 'Fix the failing CI workflow',
  ref: 'main',
  target: {
    autoCreatePr: true,
    branchName: 'fix/ci-workflow',
  },
});

if (result.success && result.data) {
  console.log(`Spawned agent: ${result.data.id}`);
}
```

---

### waitFor()

Wait for an agent to complete.

```typescript
waitFor(agentId: string, options?: WaitOptions): Promise<Result<Agent>>
```

**Parameters:**

```typescript
interface WaitOptions {
  timeout?: number;      // Milliseconds (default: 600000 / 10 min)
  pollInterval?: number; // Milliseconds (default: 15000 / 15 sec)
}
```

**Returns:** `Result<Agent>` - The completed agent.

**Example:**
```typescript
const result = await fleet.waitFor(agent.id, {
  timeout: 1800000,  // 30 minutes
  pollInterval: 30000,  // 30 seconds
});

if (result.success && result.data) {
  console.log(`Agent finished: ${result.data.status}`);
}
```

---

### conversation()

Get an agent's conversation.

```typescript
conversation(agentId: string): Promise<Result<Conversation>>
```

**Returns:** `Result<Conversation>`

```typescript
interface Conversation {
  agentId: string;
  totalMessages: number;
  messages: Message[];
}

interface Message {
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp?: string;
}
```

**Example:**
```typescript
const result = await fleet.conversation(agent.id);
if (result.success && result.data) {
  console.log(`${result.data.totalMessages} messages`);
  for (const msg of result.data.messages) {
    console.log(`[${msg.role}] ${msg.content.slice(0, 100)}...`);
  }
}
```

---

### followup()

Send a followup message to an agent.

```typescript
followup(agentId: string, message: string): Promise<Result<void>>
```

**Example:**
```typescript
const result = await fleet.followup(
  agent.id,
  'Please focus on the authentication module first'
);

if (result.success) {
  console.log('Message sent');
}
```

---

### broadcast()

Send a message to multiple agents.

```typescript
broadcast(agentIds: string[], message: string): Promise<Map<string, Result<void>>>
```

**Returns:** `Map<string, Result<void>>` - Results for each agent.

**Example:**
```typescript
const results = await fleet.broadcast(
  ['agent-1', 'agent-2', 'agent-3'],
  'STATUS CHECK: Please provide a progress update.'
);

for (const [id, result] of results) {
  console.log(`${id}: ${result.success ? 'sent' : result.error}`);
}
```

---

### archive()

Archive an agent's conversation to disk.

```typescript
archive(agentId: string, path?: string): Promise<Result<string>>
```

**Returns:** `Result<string>` - Path to the saved file.

**Example:**
```typescript
const result = await fleet.archive(agent.id);
if (result.success) {
  console.log(`Saved to: ${result.data}`);
}

// Custom path
const result2 = await fleet.archive(agent.id, './archives/agent.json');
```

---

### repositories()

List available repositories.

```typescript
repositories(): Promise<Result<Repository[]>>
```

**Returns:** `Result<Repository[]>`

```typescript
interface Repository {
  fullName: string;
  defaultBranch: string;
  isPrivate: boolean;
  url: string;
}
```

**Example:**
```typescript
const result = await fleet.repositories();
if (result.success && result.data) {
  for (const repo of result.data) {
    console.log(`${repo.fullName} (${repo.defaultBranch})`);
  }
}
```

---

### listModels()

List available AI models.

```typescript
listModels(): Promise<Result<string[]>>
```

**Returns:** `Result<string[]>` - Available model names.

**Example:**
```typescript
const result = await fleet.listModels();
if (result.success && result.data) {
  console.log('Available models:', result.data.join(', '));
}
```

---

### monitorAgents()

Monitor multiple agents with callbacks.

```typescript
monitorAgents(
  agentIds: string[],
  options?: MonitorOptions
): Promise<Map<string, Agent>>
```

**Parameters:**

```typescript
interface MonitorOptions {
  pollInterval?: number;  // Milliseconds (default: 30000)
  onProgress?: (statusMap: Map<string, AgentStatus>) => void;
}
```

**Returns:** `Map<string, Agent>` - Final states of all agents.

**Example:**
```typescript
const results = await fleet.monitorAgents(
  ['agent-1', 'agent-2'],
  {
    pollInterval: 30000,
    onProgress: (statusMap) => {
      for (const [id, status] of statusMap) {
        console.log(`${id}: ${status}`);
      }
    },
  }
);

for (const [id, agent] of results) {
  console.log(`Final: ${id} -> ${agent.status}`);
}
```

---

### createDiamond()

Create a diamond orchestration pattern.

```typescript
createDiamond(config: DiamondConfig): Promise<Result<DiamondResult>>
```

**Parameters:**

```typescript
interface DiamondConfig {
  targetRepos: SpawnOptions[];   // Parallel agents
  counterparty: SpawnOptions;    // Aggregator agent
  controlCenter: string;         // Coordination repo URL
}
```

**Returns:** `Result<DiamondResult>`

```typescript
interface DiamondResult {
  targetAgents: Agent[];
  counterpartyAgent: Agent;
  coordinationPr: number;
}
```

**Example:**
```typescript
const result = await fleet.createDiamond({
  targetRepos: [
    { repository: 'org/frontend', task: 'Update UI' },
    { repository: 'org/backend', task: 'Update API' },
  ],
  counterparty: {
    repository: 'org/integration',
    task: 'Verify integration',
  },
  controlCenter: 'https://github.com/org/control',
});
```

---

### coordinate()

Run coordination loop (runs indefinitely).

```typescript
coordinate(config: CoordinationConfig): Promise<void>
```

**Parameters:**

```typescript
interface CoordinationConfig {
  coordinationPr: number;      // PR number for coordination
  repo: string;                // Repo in "owner/name" format
  agentIds: string[];          // Agents to coordinate
  outboundInterval?: number;   // Check agents (ms)
  inboundInterval?: number;    // Check PR comments (ms)
}
```

**Example:**
```typescript
await fleet.coordinate({
  coordinationPr: 123,
  repo: 'my-org/control-center',
  agentIds: ['agent-1', 'agent-2'],
  outboundInterval: 60000,
  inboundInterval: 15000,
});
```

## Types

### Agent

```typescript
interface Agent {
  id: string;
  status: AgentStatus;
  source: {
    repository: string;
    ref: string;
  };
  target?: {
    autoCreatePr: boolean;
    branchName?: string;
    prUrl?: string;
  };
  task: string;
  createdAt: string;
  updatedAt: string;
  summary?: string;
}
```

### AgentStatus

```typescript
type AgentStatus = 
  | 'PENDING'
  | 'RUNNING'
  | 'COMPLETED'
  | 'FINISHED'
  | 'FAILED'
  | 'CANCELLED'
  | 'UNKNOWN';
```

### Result

```typescript
interface Result<T> {
  success: boolean;
  data?: T;
  error?: string;
}
```

## Next Steps

- [Token Management API](/api/token-management/) - Token configuration
- [Triage Tools API](/api/triage-tools/) - AI triage tools
- [TypeScript Examples](/examples/typescript/) - Code examples
