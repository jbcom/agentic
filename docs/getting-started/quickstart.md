# Quickstart

## TypeScript CLI

The `agentic` CLI provides fleet management and triage capabilities.

### Start MCP Server

```bash
npx agentic mcp
```

### Configuration

Create a `agentic.config.js` or `agentic.config.ts` file:

```typescript
import { defineConfig } from 'agentic-control';

export default defineConfig({
  fleet: {
    // Fleet configuration
  },
  triage: {
    // Triage configuration
  },
});
```

## Python CrewAI

### Running Crews

```bash
# Run the game builder crew
crew-agents game_builder

# Run with specific inputs
run-crew game_builder --game-type puzzle
```

### Running Flows

```bash
# Run the TDD prototype flow
python -m crew_agents.run_flow tdd_prototype

# Run with inputs
python -m crew_agents.run_flow game_design --input '{"game_name": "MyGame"}'
```

### Available Crews

- `game_builder` - Game building and design crew
- `creature_design` - Creature design and modeling crew
- `gameplay_design` - Gameplay mechanics design crew
- `world_design` - World building and environment design crew
- `asset_pipeline` - Asset generation pipeline crew
- `ecs_implementation` - Entity Component System implementation crew
- `qa_validation` - QA and validation crew
- `rendering` - Rendering and graphics crew

### Available Flows

- `game_design_flow` - Complete game design workflow
- `tdd_prototype_flow` - Test-driven development prototyping
- `implementation_flow` - Implementation workflow
- `asset_generation_flow` - Asset generation workflow
