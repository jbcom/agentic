---
title: "Fleet API Reference"
description: "Complete API reference for the Fleet class and Cursor Background Agent management from @jbcom/agentic-control."
---

# Fleet API Reference

The `Fleet` class is the primary interface for managing Cursor Background Agents. It provides agent lifecycle management, communication, diamond-pattern orchestration, and token-aware GitHub coordination.

## Installation

```bash
npm install @jbcom/agentic-control
```

## Import

```typescript
import { Fleet, CursorAPI } from '@jbcom/agentic-control';
import type {
  FleetConfig,
  CoordinationConfig,
  SpawnContext,
  CursorAPIOptions,
} from '@jbcom/agentic-control';
```

## Constructor

```typescript
const fleet = new Fleet(config?: FleetConfig);
```

### FleetConfig

`FleetConfig` extends `CursorAPIOptions` with fleet-specific options:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `apiKey` | `string` | `process.env.CURSOR_API_KEY` | Cursor API key |
| `timeout` | `number` | `60000` | Request timeout in milliseconds |
| `baseUrl` | `string` | `https://api.cursor.com/v0` | API base URL |
| `maxRetries` | `number` | `3` | Maximum retries for transient errors |
| `retryDelay` | `number` | `1000` | Initial retry delay (exponential backoff) |
| `archivePath` | `string` | `./memory-bank/recovery` | Path for archived conversations |

**Example:**

```typescript
import { Fleet } from '@jbcom/agentic-control';

const fleet = new Fleet({
  apiKey: process.env.CURSOR_API_KEY,
  timeout: 120000,
  maxRetries: 5,
  retryDelay: 2000,
  archivePath: './archives',
});
```

If `CURSOR_API_KEY` is not set, the Fleet instance is created but API methods return `{ success: false, error: 'Cursor API not available' }`.

---

## API Availability

### isApiAvailable()

Check if the Cursor API key is configured and the internal client is initialized.

```typescript
isApiAvailable(): boolean
```

**Returns:** `true` if `CursorAPI` was successfully initialized, `false` otherwise.

```typescript
if (!fleet.isApiAvailable()) {
  console.error('Set CURSOR_API_KEY environment variable');
  process.exit(1);
}
```

---

## Agent Discovery

### list()

List all agents in the fleet.

```typescript
list(): Promise<Result<Agent[]>>
```

**Returns:** `Result<Agent[]>` containing all agents regardless of status.

```typescript
const result = await fleet.list();
if (result.success && result.data) {
  for (const agent of result.data) {
    console.log(`${agent.id}: ${agent.status} - ${agent.source.repository}`);
  }
}
```

### listByStatus()

Filter agents by a specific status.

```typescript
listByStatus(status: AgentStatus): Promise<Result<Agent[]>>
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | `AgentStatus` | One of `RUNNING`, `FINISHED`, `COMPLETED`, `FAILED`, `CANCELLED`, `PENDING`, `UNKNOWN` |

```typescript
const failed = await fleet.listByStatus('FAILED');
if (failed.success && failed.data) {
  console.log(`${failed.data.length} agents have failed`);
}
```

### running()

Convenience method to list only running agents. Equivalent to `listByStatus('RUNNING')`.

```typescript
running(): Promise<Result<Agent[]>>
```

```typescript
const result = await fleet.running();
if (result.success && result.data) {
  console.log(`${result.data.length} agents currently running`);
}
```

### find()

Find a specific agent by ID.

```typescript
find(agentId: string): Promise<Result<Agent | undefined>>
```

```typescript
const result = await fleet.find('bc-abc123');
if (result.success && result.data) {
  console.log(`Found agent: ${result.data.status}`);
}
```

### status()

Get the current status of a specific agent. Makes a direct API call per agent (unlike `find()` which searches the full list).

```typescript
status(agentId: string): Promise<Result<Agent>>
```

```typescript
const result = await fleet.status('bc-abc123');
if (result.success && result.data) {
  console.log(`Agent status: ${result.data.status}`);
  if (result.data.target?.prUrl) {
    console.log(`PR: ${result.data.target.prUrl}`);
  }
}
```

---

## Agent Spawning

### spawn()

Spawn a new Cursor Background Agent. This is the primary method for launching agents.

```typescript
spawn(options: SpawnOptions & { context?: SpawnContext }): Promise<Result<Agent>>
```

**SpawnOptions:**

```typescript
interface SpawnOptions {
  /** GitHub repository URL (e.g., https://github.com/org/repo) */
  repository: string;
  /** Task description for the agent */
  task: string;
  /** Git ref (branch, tag, commit) - defaults to "main" */
  ref?: string;
  /** Target configuration */
  target?: {
    /** Auto-create PR when agent completes */
    autoCreatePr?: boolean;
    /** Custom branch name */
    branchName?: string;
    /** Open PR as Cursor GitHub App instead of user */
    openAsCursorGithubApp?: boolean;
    /** Skip adding user as reviewer */
    skipReviewerRequest?: boolean;
  };
  /** Webhook for status notifications */
  webhook?: {
    /** URL to receive notifications (must be HTTPS) */
    url: string;
    /** Secret for payload verification (min 32 chars) */
    secret?: string;
  };
}
```

**SpawnContext** (optional coordination context):

```typescript
interface SpawnContext {
  controlManagerId?: string;
  controlCenter?: string;
  relatedAgents?: string[];
  metadata?: Record<string, unknown>;
}
```

When `context` is provided, a `--- COORDINATION CONTEXT ---` block is appended to the task string with the manager ID, control center URL, related agent IDs, and metadata.

**Example -- basic spawn:**

```typescript
const result = await fleet.spawn({
  repository: 'https://github.com/my-org/my-repo',
  task: 'Fix the failing CI workflow in .github/workflows/ci.yml',
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

**Example -- spawn with webhook:**

```typescript
const result = await fleet.spawn({
  repository: 'https://github.com/my-org/my-repo',
  task: 'Refactor the authentication module',
  target: { autoCreatePr: true },
  webhook: {
    url: 'https://hooks.myapp.com/cursor',
    secret: 'whsec_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6',
  },
});
```

**Validation rules:**

- `repository` must contain `/` (owner/repo or full URL format), max 200 characters
- `task` cannot be empty, max 100,000 characters
- `webhook.url` must be HTTPS and cannot point to internal/private addresses (SSRF protection)
- `ref` must be under 200 characters

---

## Agent Communication

### followup()

Send a follow-up message to a running agent.

```typescript
followup(agentId: string, message: string): Promise<Result<void>>
```

```typescript
const result = await fleet.followup(
  'bc-abc123',
  'Please focus on the authentication module first, then handle the API routes.'
);

if (result.success) {
  console.log('Follow-up message sent');
}
```

### broadcast()

Send the same message to multiple agents concurrently.

```typescript
broadcast(agentIds: string[], message: string): Promise<Map<string, Result<void>>>
```

**Returns:** A `Map` where each key is an agent ID and the value is the result of sending the message.

```typescript
const results = await fleet.broadcast(
  ['bc-abc123', 'bc-def456', 'bc-ghi789'],
  'STATUS CHECK: Please provide a progress update in the coordination PR.'
);

for (const [id, result] of results) {
  console.log(`${id}: ${result.success ? 'sent' : result.error}`);
}
```

---

## Conversations

### conversation()

Retrieve the full conversation history for an agent.

```typescript
conversation(agentId: string): Promise<Result<Conversation>>
```

**Returns:** `Result<Conversation>` where:

```typescript
interface Conversation {
  agentId: string;
  messages: ConversationMessage[];
  totalMessages: number;
}

interface ConversationMessage {
  type: 'user_message' | 'assistant_message';
  text: string;
  timestamp?: string;
}
```

```typescript
const result = await fleet.conversation('bc-abc123');
if (result.success && result.data) {
  console.log(`${result.data.totalMessages} messages`);
  for (const msg of result.data.messages) {
    console.log(`[${msg.type}] ${msg.text.slice(0, 100)}...`);
  }
}
```

### archive()

Archive an agent's conversation to a JSON file on disk.

```typescript
archive(agentId: string, outputPath?: string): Promise<Result<string>>
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `agentId` | `string` | -- | Agent ID to archive |
| `outputPath` | `string` | `{archivePath}/conversation-{agentId}.json` | Custom output path |

**Returns:** `Result<string>` containing the path to the saved file.

```typescript
// Default path
const result = await fleet.archive('bc-abc123');
if (result.success) {
  console.log(`Saved to: ${result.data}`);
  // -> ./memory-bank/recovery/conversation-bc-abc123.json
}

// Custom path
const result2 = await fleet.archive('bc-abc123', './archives/agent.json');
```

---

## Repositories and Models

### repositories()

List all repositories available to the Cursor API account.

```typescript
repositories(): Promise<Result<Repository[]>>
```

```typescript
interface Repository {
  owner: string;
  name: string;
  fullName: string;
  defaultBranch: string;
  isPrivate: boolean;
  url: string;
}
```

```typescript
const result = await fleet.repositories();
if (result.success && result.data) {
  for (const repo of result.data) {
    console.log(`${repo.fullName} (${repo.defaultBranch})`);
  }
}
```

### listModels()

List available AI models for agent execution.

```typescript
listModels(): Promise<Result<string[]>>
```

```typescript
const result = await fleet.listModels();
if (result.success && result.data) {
  console.log('Available models:', result.data.join(', '));
}
```

---

## Fleet Monitoring

### summary()

Get aggregate statistics across all agents.

```typescript
summary(): Promise<Result<FleetSummary>>
```

**Returns:**

```typescript
interface FleetSummary {
  total: number;
  running: number;
  completed: number;   // Includes both COMPLETED and FINISHED statuses
  failed: number;
  agents: Agent[];
}
```

```typescript
const result = await fleet.summary();
if (result.success && result.data) {
  const { total, running, completed, failed } = result.data;
  console.log(`Fleet: ${total} total, ${running} running, ${completed} done, ${failed} failed`);
}
```

### waitFor()

Block until an agent reaches a terminal state (anything other than `RUNNING`).

```typescript
waitFor(agentId: string, options?: WaitOptions): Promise<Result<Agent>>
```

**WaitOptions:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `timeout` | `number` | `300000` (5 min) | Maximum wait time in milliseconds |
| `pollInterval` | `number` | `10000` (10 sec) | Polling interval in milliseconds |

```typescript
const result = await fleet.waitFor('bc-abc123', {
  timeout: 1800000,    // 30 minutes
  pollInterval: 30000, // 30 seconds
});

if (result.success && result.data) {
  console.log(`Agent finished with status: ${result.data.status}`);
} else {
  console.error(`Wait failed: ${result.error}`);
  // error is "Timeout waiting for agent bc-abc123" on timeout
}
```

### monitorAgents()

Monitor multiple agents until all reach terminal states. Supports progress callbacks.

```typescript
monitorAgents(
  agentIds: string[],
  options?: MonitorOptions
): Promise<Map<string, Agent>>
```

**MonitorOptions:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `pollInterval` | `number` | `15000` (15 sec) | Polling interval |
| `onProgress` | `(status: Map<string, AgentStatus>) => void` | -- | Called each poll cycle |

Terminal states are any status other than `RUNNING` or `PENDING`.

```typescript
const results = await fleet.monitorAgents(
  ['bc-abc123', 'bc-def456'],
  {
    pollInterval: 30000,
    onProgress: (statusMap) => {
      for (const [id, status] of statusMap) {
        console.log(`${id.slice(0, 12)}: ${status}`);
      }
    },
  }
);

for (const [id, agent] of results) {
  console.log(`Final: ${id} -> ${agent.status}`);
}
```

---

## Diamond Pattern Orchestration

### createDiamond()

Create a diamond orchestration pattern: spawn multiple target agents in parallel, then a counterparty agent that knows about all targets.

```typescript
createDiamond(config: DiamondConfig): Promise<Result<DiamondResult>>
```

**DiamondConfig:**

```typescript
interface DiamondConfig {
  /** Parallel target agents */
  targetRepos: SpawnOptions[];
  /** Aggregator/verifier agent */
  counterparty: SpawnOptions;
  /** Control center repository URL */
  controlCenter: string;
}
```

**DiamondResult:**

```typescript
interface DiamondResult {
  targetAgents: Agent[];
  counterpartyAgent: Agent;
}
```

The diamond pattern works as follows:

1. Identifies the current control manager agent ID
2. Spawns all `targetRepos` agents with coordination context (control manager ID, control center)
3. Spawns the `counterparty` agent with knowledge of all target agent IDs and the `diamond` pattern metadata
4. Sends follow-up messages to all target agents informing them of the counterparty agent

```typescript
const result = await fleet.createDiamond({
  targetRepos: [
    {
      repository: 'https://github.com/org/frontend',
      task: 'Update the UI components for the new API',
      target: { autoCreatePr: true },
    },
    {
      repository: 'https://github.com/org/backend',
      task: 'Add the new REST endpoints',
      target: { autoCreatePr: true },
    },
  ],
  counterparty: {
    repository: 'https://github.com/org/integration-tests',
    task: 'Verify end-to-end integration between frontend and backend changes',
    target: { autoCreatePr: true },
  },
  controlCenter: 'https://github.com/org/control',
});

if (result.success && result.data) {
  console.log(`Spawned ${result.data.targetAgents.length} target agents`);
  console.log(`Counterparty: ${result.data.counterpartyAgent.id}`);
}
```

---

## GitHub Coordination

### coordinate()

Run a bidirectional coordination loop that bridges PR comments with agent follow-up messages. This method runs indefinitely.

```typescript
coordinate(config: CoordinationConfig): Promise<void>
```

**CoordinationConfig:**

```typescript
interface CoordinationConfig {
  /** PR number for coordination channel */
  coordinationPr: number;
  /** Repository in owner/repo format */
  repo: string;
  /** Outbound poll interval (ms) - check agents and send status requests */
  outboundInterval?: number;  // default: 60000
  /** Inbound poll interval (ms) - check PR comments for @cursor mentions */
  inboundInterval?: number;   // default: 15000
  /** Agent IDs to monitor */
  agentIds?: string[];
}
```

The coordination loop runs two concurrent processes:

- **Outbound loop**: Periodically checks each agent's status. If running, sends a follow-up requesting a progress update. If finished, removes the agent from the monitoring set.
- **Inbound loop**: Watches for new PR comments containing `@cursor`. Processes structured coordination messages like `DONE: <agent-id> <summary>` and `BLOCKED: <agent-id> <issue>`.

```typescript
await fleet.coordinate({
  coordinationPr: 42,
  repo: 'my-org/control-center',
  agentIds: ['bc-abc123', 'bc-def456'],
  outboundInterval: 60000,
  inboundInterval: 15000,
});
```

Token routing is automatic -- the `GitHubClient` uses the organization extracted from `repo` to select the correct GitHub token via the [Token Management](/api/token-management/) system.

---

## Types

### Agent

```typescript
interface Agent {
  id: string;
  name?: string;
  status: AgentStatus;
  source: AgentSource;
  target?: AgentTarget;
  createdAt?: string;
  updatedAt?: string;
  summary?: string;
  error?: string;
}

interface AgentSource {
  repository: string;
  ref?: string;
  commitSha?: string;
}

interface AgentTarget {
  branchName?: string;
  url?: string;
  prUrl?: string;
  prNumber?: number;
}
```

### AgentStatus

```typescript
type AgentStatus =
  | 'RUNNING'     // Agent is actively working
  | 'FINISHED'    // Agent completed work (Cursor API term)
  | 'COMPLETED'   // Agent completed work (alias)
  | 'FAILED'      // Agent encountered an error
  | 'CANCELLED'   // Agent was cancelled
  | 'PENDING'     // Agent is queued but not started
  | 'UNKNOWN';    // Status could not be determined
```

### Result

All Fleet methods return a `Result<T>` wrapper:

```typescript
interface Result<T> {
  success: boolean;
  data?: T;
  error?: string;
}
```

---

## Error Handling

The Fleet class never throws exceptions from API operations. All errors are captured in the `Result` wrapper. The underlying `CursorAPI` client handles:

- **Retryable errors**: HTTP 429, 500, 502, 503, 504 are retried with exponential backoff (up to `maxRetries` times)
- **Timeout errors**: Retried automatically
- **Network errors**: `TypeError`, connection failures are retried
- **Input validation**: Invalid agent IDs, repository formats, or webhook URLs throw synchronously before the API call
- **Sensitive data**: Error messages are sanitized to redact API keys and tokens

```typescript
const result = await fleet.spawn({
  repository: 'https://github.com/org/repo',
  task: 'Fix the bug',
});

if (!result.success) {
  // result.error contains a safe, redacted error message
  console.error('Spawn failed:', result.error);
  // e.g., "API Error 401: Unauthorized"
  // e.g., "Cursor API not available"
  // e.g., "Request timeout after 60000ms"
}
```

---

## CursorAPI (Low-Level)

For direct API access without Fleet's high-level features, use `CursorAPI`:

```typescript
import { CursorAPI } from '@jbcom/agentic-control';

const api = new CursorAPI({
  apiKey: process.env.CURSOR_API_KEY,
});

// Check availability without instantiating
if (CursorAPI.isAvailable()) {
  const agents = await api.listAgents();
  const models = await api.listModels();
}
```

---

## Related Pages

- [Token Management API](/api/token-management/) -- Token configuration and multi-org routing
- [Triage Tools API](/api/triage-tools/) -- AI-powered issue and PR triage
- [Configuration API](/api/configuration/) -- Full config schema reference
- [TypeScript Examples](/examples/typescript/) -- Code examples
