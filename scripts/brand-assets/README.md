# Brand Asset Generation

Dogfooding the `meshy-content-generator` package to produce Agentic brand imagery.

## Prerequisites

- Node.js 20+
- `MESHY_API_KEY` environment variable (available via Doppler)

## What It Generates

| Asset | Directory | Aspect Ratio | Description |
|-------|-----------|--------------|-------------|
| Hero Image | `hero-image/` | 16:9 | Abstract command-center visualization for landing pages |
| OG Background | `og-background/` | 16:9 | Dark textured background for Open Graph / social cards |
| Icon Concept | `icon-concept/` | 1:1 | Geometric logo mark exploration |

Each directory contains a `manifest.json` that drives the generation and stores state.

## Running

```bash
# Option 1: Export the key directly
export MESHY_API_KEY="your-key-here"
bash scripts/brand-assets/generate.sh

# Option 2: Use Doppler to inject secrets
doppler run -- bash scripts/brand-assets/generate.sh
```

The script iterates over every subdirectory that contains a `manifest.json` and runs the `brand-image` pipeline against it.

## How Idempotent Manifests Work

Each `manifest.json` starts as a plain asset description. When the pipeline runs:

1. A deterministic `seed` is written into the manifest (first run only).
2. The Meshy API task is created and polled until completion.
3. Task state (`taskId`, `status`, `outputs`, `artifacts`) is persisted back into the manifest under the `tasks` key.
4. Generated images are downloaded as `concept-0.png`, `concept-1.png`, etc.

On subsequent runs the runner detects `"status": "SUCCEEDED"` in the manifest and skips the API call entirely. If an artifact file is missing on disk but the task succeeded, only the download step is re-executed.

To force regeneration, delete the `tasks` key from the manifest and remove the local image files.

## Customizing Prompts

Edit the `textToImageTask.prompt` field in any `manifest.json`. Other tunable fields:

- `aiModel` -- Meshy model variant (default: `nano-banana`)
- `aspectRatio` -- `1:1`, `16:9`, `4:3`, etc.
- `generateMultiView` -- set `true` for multi-view concept sheets

After editing, re-run `generate.sh`. If the previous task state is still present in the manifest, delete the `tasks` key first so the pipeline treats it as a fresh run.

## Adding New Assets

1. Create a new subdirectory under `scripts/brand-assets/`.
2. Add a `manifest.json` following the schema (must have `id`, `name`, `type`, and a `textToImageTask` block).
3. Run `generate.sh` -- the new directory will be picked up automatically.
