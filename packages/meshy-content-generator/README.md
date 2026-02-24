# meshy-content-generator

Ship Meshy-powered 3D asset pipelines without bespoke scripts. Define pipelines and tasks as JSON, run them from a CLI or API, and preview results instantly.

## Why teams adopt it

- **Meshy-first**: built around Meshy’s text-to-image, text-to-3D, rigging, and animation endpoints.
- **Declarative pipelines**: orchestration lives in JSON, not in brittle glue code.
- **Reusable assets**: one manifest per asset; replay the exact pipeline anytime.
- **CLI + API + Preview**: run locally, in CI, or embed as a library.
- **Testable**: VCR recording for Meshy calls, contract tests for OpenAPI.

## Install

```bash
pnpm add @jbcom/agentic-meshy
```

## Quick start (CLI)

```bash
# list built-in pipelines and tasks
content-gen list --pipelines ./pipelines/definitions --tasks ./tasks/definitions

# validate an asset manifest
content-gen validate ./assets/characters/hero \
  --pipelines ./pipelines/definitions \
  --tasks ./tasks/definitions

# run a pipeline
content-gen run character ./assets/characters/hero \
  --pipelines ./pipelines/definitions \
  --tasks ./tasks/definitions
```

### Environment

```bash
MESHY_API_KEY=your_api_key
POLLY_MODE=replay
```

## Programmatic API

```ts
import { PipelineRunner, loadJsonDefinitions } from "@jbcom/agentic-meshy";

const definitions = await loadJsonDefinitions({
  pipelinesDir: "./pipelines/definitions",
  tasksDir: "./tasks/definitions",
});

const runner = new PipelineRunner({
  definitions,
  apiKey: process.env.MESHY_API_KEY!,
});

await runner.run({
  pipelineName: "character",
  assetDir: "./assets/characters/hero",
});
```

## API server + preview

```bash
pnpm dev
```

- API reference: `http://localhost:5177/api`
- OpenAPI spec: `http://localhost:5177/openapi.json`
- Preview: `http://localhost:5177/preview?assetDir=./assets/characters/hero&file=model.glb`

The preview loads `@google/model-viewer` from your local install (no CDN).

## Concepts

### Pipeline definitions

`pipelines/definitions/*.pipeline.json` describe **orchestration**. Each step references a task and can override inputs.

### Task definitions

`tasks/definitions/*.json` describe **Meshy calls**. Inputs resolve from manifests, previous steps, literals, env vars, or lookup tables.

### Manifest

Each asset directory contains a `manifest.json` that supplies task inputs and stores task state.

## Testing

```bash
pnpm test:unit
pnpm test:e2e
pnpm test:all
```

To record Meshy calls once and replay in CI:

```bash
POLLY_MODE=record pnpm test:unit
```

## Scripts

```bash
pnpm check
pnpm test
pnpm build
```

## Documentation

The docs site is built with Astro + Starlight. Source content lives in `src/content/docs`.

## Security

- Only serve the API locally unless you add authentication.
- The preview endpoint serves files only within the working directory.

## License

MIT
