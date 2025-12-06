# Crew Tool Integration Architecture

## Overview

The crew tool integration enables TypeScript components (Fleet, Triage) to invoke Python CrewAI crews as tools via subprocess execution, creating a symbiotic relationship between the two systems.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    TypeScript (agentic-control)              │
│                                                              │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐         │
│  │  Fleet   │      │  Triage  │      │  GitHub  │         │
│  │ Manager  │      │ Analyzer │      │  Client  │         │
│  └────┬─────┘      └────┬─────┘      └──────────┘         │
│       │                 │                                   │
│       └────────┬────────┘                                   │
│                │                                            │
│         ┌──────▼──────┐                                     │
│         │  CrewTool   │                                     │
│         │  Interface  │                                     │
│         └──────┬──────┘                                     │
│                │                                            │
└────────────────┼────────────────────────────────────────────┘
                 │
                 │ subprocess spawn
                 │ (Node.js child_process)
                 │
┌────────────────▼────────────────────────────────────────────┐
│                    Python (crew-agents)                      │
│                                                              │
│         ┌──────────────┐                                    │
│         │  CLI Entry   │                                    │
│         │    Point     │                                    │
│         └──────┬───────┘                                    │
│                │                                            │
│         ┌──────▼───────┐                                    │
│         │   Discovery  │                                    │
│         │    Engine    │                                    │
│         └──────┬───────┘                                    │
│                │                                            │
│         ┌──────▼───────┐                                    │
│         │     Crew     │                                    │
│         │    Runner    │                                    │
│         └──────┬───────┘                                    │
│                │                                            │
│    ┌───────────┴───────────┐                               │
│    │                       │                               │
│ ┌──▼────┐  ┌──────┐  ┌────▼───┐                           │
│ │ Crew1 │  │ Crew2│  │ Crew3  │                           │
│ └───────┘  └──────┘  └────────┘                           │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

### TypeScript Layer

#### CrewTool
- **Purpose**: Main interface for crew invocation
- **Responsibilities**:
  - Configuration management and validation
  - Subprocess spawning and lifecycle management
  - Timeout handling
  - Error categorization and serialization
  - Output parsing

#### Fleet Integration
- **Purpose**: Spawn agents with crew-generated specifications
- **Method**: `spawnWithCrewSpec()`
- **Flow**:
  1. Invoke crew to generate specification
  2. Parse crew output
  3. Spawn agent with generated spec
  4. Return agent handle

#### Triage Integration
- **Purpose**: Delegate analysis tasks to specialized crews
- **Method**: `delegateToCrew()`
- **Flow**:
  1. Invoke crew with analysis request
  2. Parse crew output
  3. Return analysis results
  4. Optionally create GitHub issues

### Python Layer

#### CLI Entry Point
- **Purpose**: Command-line interface for crew operations
- **Commands**:
  - `list`: List available crews
  - `info <package> <crew>`: Get crew information
  - `run <package> <crew> --input <input>`: Execute crew

#### Discovery Engine
- **Purpose**: Find and catalog available crews
- **Process**:
  1. Scan for `.crewai/` directories
  2. Parse `manifest.yaml` files
  3. Build crew registry
  4. Return crew metadata

#### Crew Runner
- **Purpose**: Execute crew with inputs
- **Process**:
  1. Load crew configuration
  2. Initialize agents and tasks
  3. Execute crew workflow
  4. Return results

## Communication Flow

### Successful Invocation

```
TypeScript                    Subprocess                    Python
    │                             │                            │
    │ spawn('uv run crew-agents') │                            │
    ├────────────────────────────>│                            │
    │                             │ execute CLI                │
    │                             ├───────────────────────────>│
    │                             │                            │
    │                             │                    discover crews
    │                             │                    load config
    │                             │                    execute crew
    │                             │                            │
    │                             │<───────────────────────────┤
    │                             │        stdout: result      │
    │<────────────────────────────┤                            │
    │   parse output              │                            │
    │   return CrewResult         │                            │
    │                             │                            │
```

### Error Handling

```
TypeScript                    Subprocess                    Python
    │                             │                            │
    │ spawn('uv run crew-agents') │                            │
    ├────────────────────────────>│                            │
    │                             │ execute CLI                │
    │                             ├───────────────────────────>│
    │                             │                            │
    │                             │                    crew error
    │                             │                            │
    │                             │<───────────────────────────┤
    │                             │    stderr: error details   │
    │                             │    exit code: 1            │
    │<────────────────────────────┤                            │
    │   categorize error          │                            │
    │   return CrewResult         │                            │
    │   (success: false)          │                            │
    │                             │                            │
```

### Timeout Handling

```
TypeScript                    Subprocess                    Python
    │                             │                            │
    │ spawn('uv run crew-agents') │                            │
    ├────────────────────────────>│                            │
    │ setTimeout(timeout)         │                            │
    │                             │ execute CLI                │
    │                             ├───────────────────────────>│
    │                             │                            │
    │                             │                    long-running
    │                             │                    crew...
    │ timeout expires             │                            │
    │ proc.kill('SIGTERM')        │                            │
    ├────────────────────────────>│                            │
    │                             │ terminate                  │
    │                             ├───────────────────────────>│
    │                             │                            │
    │<────────────────────────────┤                            │
    │   partial output            │                            │
    │   return CrewResult         │                            │
    │   (error: 'timed out')      │                            │
    │                             │                            │
```

## Data Flow

### Configuration Flow

```
agentic.config.json
        │
        ▼
  initConfig()
        │
        ▼
  getCrewsConfig()
        │
        ▼
  CrewTool constructor
        │
        ▼
  validateConfig()
        │
        ▼
  Merge with defaults
        │
        ▼
  Store in CrewTool.config
```

### Invocation Flow

```
User Code
    │
    ▼
invokeCrew(options)
    │
    ▼
validateInvokeOptions()
    │
    ▼
Build command args
    │
    ▼
Merge environment vars
    │
    ▼
spawn subprocess
    │
    ▼
Set timeout handler
    │
    ▼
Collect stdout/stderr
    │
    ▼
Wait for exit
    │
    ▼
Parse output
    │
    ▼
Return CrewResult
```

## Error Categorization

```
Error Source          Category          Handled By
─────────────────────────────────────────────────────
Invalid config    →   config        →   Constructor
Invalid params    →   validation    →   invokeCrew()
Spawn failure     →   subprocess    →   spawn handler
Python not found  →   subprocess    →   spawn handler
Crew not found    →   crew          →   exit code check
Crew exception    →   crew          →   stderr parsing
Timeout           →   crew          →   timeout handler
Parse failure     →   communication →   output parser
```

## Security Considerations

### Input Validation
- Package names: `^[a-zA-Z0-9_-]+$`
- Crew names: `^[a-zA-Z0-9_-]+$`
- Input strings: No restrictions (passed as-is)
- Timeout values: Must be positive integers

### Subprocess Isolation
- No shell execution (uses spawn, not exec)
- Environment variables explicitly passed
- Working directory controlled
- Process killed on timeout

### Error Information
- Stack traces sanitized
- Sensitive data not logged
- Error messages user-friendly
- Details available for debugging

## Performance Characteristics

### Latency
- **Subprocess spawn**: ~50-200ms
- **Crew discovery**: ~100-500ms
- **Crew execution**: Varies (seconds to minutes)
- **Total overhead**: ~150-700ms + crew time

### Resource Usage
- **Memory**: Subprocess isolated
- **CPU**: Depends on crew complexity
- **I/O**: Stdout/stderr streaming
- **Network**: Crew-dependent (API calls)

### Optimization Strategies
1. **Lazy Initialization**: CrewTool loaded on-demand
2. **Configuration Caching**: Config loaded once
3. **Subprocess Reuse**: Not implemented (stateless)
4. **Output Streaming**: Real-time stdout/stderr
5. **Timeout Management**: Configurable per-invocation

## Scalability

### Concurrent Invocations
- Multiple crews can run in parallel
- Each invocation spawns separate subprocess
- No shared state between invocations
- Limited by system resources

### Example: Parallel Execution
```typescript
const results = await Promise.all([
  crewTool.invokeCrew({ package: 'pkg1', crew: 'crew1', input: 'task1' }),
  crewTool.invokeCrew({ package: 'pkg2', crew: 'crew2', input: 'task2' }),
  crewTool.invokeCrew({ package: 'pkg3', crew: 'crew3', input: 'task3' }),
]);
```

## Integration Patterns

### Pattern 1: Direct Invocation
```typescript
const crewTool = new CrewTool(config);
const result = await crewTool.invokeCrew(options);
```

### Pattern 2: Fleet Integration
```typescript
const fleet = new Fleet();
const result = await fleet.spawnWithCrewSpec(repo, pkg, crew, input);
```

### Pattern 3: Triage Integration
```typescript
const analyzer = new Analyzer();
const result = await analyzer.delegateToCrew(pkg, crew, input);
```

### Pattern 4: Chained Crews
```typescript
const design = await crewTool.invokeCrew({ ... });
const impl = await crewTool.invokeCrew({ 
  input: design.output 
});
```

## Testing Strategy

### Unit Tests
- Configuration validation
- Input validation
- Error categorization
- Timeout handling

### Property-Based Tests
- Configuration validation (Property 7)
- Subprocess execution (Property 4)
- Timeout handling (Properties 11, 12)
- Environment variables (Property 6)
- Input validation (Properties 8, 9)
- Error handling (Properties 13, 14)
- Crew discovery (Property 5)

### Integration Tests
- End-to-end crew invocation
- Fleet integration
- Triage integration
- Error propagation

## Deployment Considerations

### Development
- Use `uv` for fast Python execution
- Enable verbose logging
- Set longer timeouts
- Use local Python environment

### Production
- Use absolute Python paths
- Disable verbose logging
- Set appropriate timeouts
- Use virtual environments
- Monitor crew execution metrics

### CI/CD
- Install Python dependencies
- Set shorter timeouts
- Use headless execution
- Cache Python environment

## Future Enhancements

### Planned
1. **Result Caching**: Cache crew outputs
2. **Execution Metrics**: Track performance
3. **Health Checks**: Verify Python environment
4. **Output Streaming**: Real-time progress

### Considered
1. **Subprocess Pooling**: Reuse Python processes
2. **Async Execution**: Non-blocking invocations
3. **Result Validation**: Schema-based output validation
4. **Retry Policies**: Automatic retry on failure

## See Also

- [Fleet with Crew Spec](../examples/integration/fleet-with-crew-spec.md)
- [Triage with Crew Delegation](../examples/integration/triage-with-crew-delegation.md)
- [Error Handling Guide](../examples/integration/error-handling.md)
- [Configuration Guide](../examples/integration/configuration.md)
