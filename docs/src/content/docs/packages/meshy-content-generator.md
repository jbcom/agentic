---
title: "@agentic/meshy"
description: Declarative 3D asset pipelines powered by Meshy AI — text-to-3D, rigging, and animation without bespoke scripts
---

# @agentic/meshy

> Declarative 3D asset pipelines powered by Meshy AI — define pipelines in JSON, run from CLI or API, preview instantly.

<div class="polyglot-bar">
  <span class="lang-badge lang-badge--ts">TypeScript</span>
</div>

**Stop writing bespoke scripts for every 3D asset.** @agentic/meshy wraps Meshy's text-to-image, text-to-3D, rigging, and animation endpoints in a declarative pipeline system. Define your pipeline as JSON, point it at an asset manifest, and get reproducible results every time.

## Why @agentic/meshy?

| Problem | Solution |
|---------|----------|
| Meshy integration requires custom scripts per asset | Declarative JSON pipelines — no code |
| Pipeline state lives in memory, lost on failure | Manifest-based state — resume from any step |
| No way to replay exact pipeline in CI | VCR recording for Meshy calls, fully deterministic |
| Preview requires external tools | Built-in 3D preview with `@google/model-viewer` |

## Installation

```bash
pnpm add @jbcom/agentic-meshy
# or
npm install @jbcom/agentic-meshy
```

## Quick Start

### CLI

```bash
# List built-in pipelines and tasks
content-gen list \
  --pipelines ./pipelines/definitions \
  --tasks ./tasks/definitions

# Validate an asset manifest
content-gen validate ./assets/characters/hero \
  --pipelines ./pipelines/definitions \
  --tasks ./tasks/definitions

# Run a pipeline
content-gen run character ./assets/characters/hero \
  --pipelines ./pipelines/definitions \
  --tasks ./tasks/definitions
```

### Programmatic API

```typescript
import {
  PipelineRunner,
  loadJsonDefinitions
} from '@jbcom/agentic-meshy';

const definitions = await loadJsonDefinitions({
  pipelinesDir: './pipelines/definitions',
  tasksDir: './tasks/definitions',
});

const runner = new PipelineRunner({
  definitions,
  apiKey: process.env.MESHY_API_KEY!,
});

await runner.run({
  pipelineName: 'character',
  assetDir: './assets/characters/hero',
});
```

## Core Concepts

### Pipeline Definitions

`pipelines/definitions/*.pipeline.json` describe **orchestration**. Each step references a task and can override inputs:

```json
{
  "name": "character",
  "steps": [
    {
      "task": "text-to-image",
      "inputs": { "prompt": "{{manifest.description}}" }
    },
    {
      "task": "image-to-3d",
      "inputs": { "image": "{{steps.0.output}}" }
    },
    {
      "task": "auto-rig",
      "inputs": { "model": "{{steps.1.output}}" }
    }
  ]
}
```

### Task Definitions

`tasks/definitions/*.json` describe **Meshy API calls**. Inputs resolve from manifests, previous steps, literals, environment variables, or lookup tables:

```json
{
  "name": "text-to-image",
  "endpoint": "/text-to-image",
  "method": "POST",
  "inputs": {
    "prompt": { "source": "manifest", "key": "description" },
    "style": { "source": "literal", "value": "realistic" }
  }
}
```

### Asset Manifests

Each asset directory contains a `manifest.json` that supplies task inputs and stores pipeline state:

```json
{
  "name": "Hero Character",
  "description": "A medieval knight with ornate armor",
  "style": "fantasy",
  "state": {
    "lastStep": 1,
    "outputs": { "0": "image_abc123.png" }
  }
}
```

## API Server & Preview

```bash
pnpm dev
```

- **API reference**: `http://localhost:5177/api`
- **OpenAPI spec**: `http://localhost:5177/openapi.json`
- **3D Preview**: `http://localhost:5177/preview?assetDir=./assets/characters/hero&file=model.glb`

The preview uses `@google/model-viewer` from your local install (no CDN dependency).

## Testing

```bash
# Unit tests
pnpm test:unit

# E2E tests
pnpm test:e2e

# Record Meshy calls for replay in CI
POLLY_MODE=record pnpm test:unit

# Replay recorded calls (default)
POLLY_MODE=replay pnpm test:unit
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `MESHY_API_KEY` | Yes | Meshy API key |
| `POLLY_MODE` | No | `record` or `replay` (default: `replay`) |

## Integration with Strata

@agentic/meshy powers the 3D asset pipeline for [Strata](https://strata.game), the AI-powered gaming platform:

- **Character models**: Text description → 3D model → rigged → animated
- **Environment assets**: Procedural generation of buildings, terrain features, props
- **Item generation**: Weapons, armor, consumables from text descriptions
- **Batch processing**: Generate entire game asset libraries from a manifest directory

## Related Packages

- **[game-generator](/packages/game-generator/)** — Uses meshy-generated assets in Bevy games
- **[@jbcom/agentic](/packages/control/)** — Fleet management and orchestration
- **[agentic-crew](/packages/crew/)** — Multi-agent workflows for content generation

## Links

- [GitHub Repository](https://github.com/jbcom/agentic/tree/main/packages/meshy-content-generator)
- [Meshy API Docs](https://docs.meshy.ai)
- [npm Package](https://www.npmjs.com/package/@jbcom/agentic-meshy)
