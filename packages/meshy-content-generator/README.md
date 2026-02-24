# @jbcom/agentic-meshy

[![npm version](https://img.shields.io/npm/v/@jbcom/agentic-meshy.svg)](https://www.npmjs.com/package/@jbcom/agentic-meshy)
[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Declarative Meshy 3D asset generation pipelines. Define text-to-image, text-to-3D, rigging, and animation workflows as JSON, then run them from a CLI, REST API, or programmatically as a library. Includes a built-in 3D model preview server powered by `@google/model-viewer`.

[Full Documentation](https://agentic.coach) | [Package Docs](https://agentic.coach/packages/meshy-content-generator/)

## Installation

```bash
npm install @jbcom/agentic-meshy
# or
pnpm add @jbcom/agentic-meshy
```

## Quick Start

### CLI

```bash
# List built-in pipelines and tasks
content-gen list --pipelines ./pipelines/definitions --tasks ./tasks/definitions

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
import { PipelineRunner, loadJsonDefinitions } from '@jbcom/agentic-meshy';

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

### API Server and Preview

```bash
pnpm dev
```

- API reference: `http://localhost:5177/api`
- OpenAPI spec: `http://localhost:5177/openapi.json`
- 3D preview: `http://localhost:5177/preview?assetDir=./assets/characters/hero&file=model.glb`

## Key Features

- **Meshy-first** -- built around Meshy's text-to-image, text-to-3D, rigging, and animation endpoints
- **Declarative pipelines** -- orchestration defined in JSON, not brittle scripts
- **Reusable manifests** -- one manifest per asset, replay the exact pipeline anytime
- **CLI + API + library** -- use from the command line, embed in a server, or import directly
- **VCR-tested** -- Polly.js recording for deterministic Meshy API tests
- **OpenAPI spec** -- auto-generated spec with Scalar API reference

## Environment

```bash
MESHY_API_KEY=your_api_key   # Required for Meshy API calls
POLLY_MODE=replay            # VCR mode: record | replay | passthrough
```

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation including the [Meshy Content Generator guide](https://agentic.coach/packages/meshy-content-generator/).

## License

MIT
