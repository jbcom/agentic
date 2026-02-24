import path from "node:path";
import { describe, expect, it } from "vitest";
import { loadJsonDefinitions } from "../core/definitions.js";
import { validateManifestAndPipeline } from "../core/validator.js";
import { loadAnimationIds } from "../lookups/animation-ids.js";
import { readJson } from "../utils/json.js";

const baseDir = process.cwd();
const manifestPath = path.join(baseDir, "test/fixtures/character-full/manifest.json");

describe("validateManifestAndPipeline", () => {
  it("validates the sample character pipeline manifest", async () => {
    const definitions = await loadJsonDefinitions({
      pipelinesDir: path.join(baseDir, "pipelines/definitions"),
      tasksDir: path.join(baseDir, "tasks/definitions"),
    });
    const manifest = readJson(manifestPath);

    const result = validateManifestAndPipeline({
      definitions,
      pipelineName: "character-full",
      manifest,
      lookups: {
        ANIMATION_IDS: loadAnimationIds(),
      },
    });

    expect(result.errors).toEqual([]);
  });

  it("fails when manifest schema is invalid", async () => {
    const definitions = await loadJsonDefinitions({
      pipelinesDir: path.join(baseDir, "pipelines/definitions"),
      tasksDir: path.join(baseDir, "tasks/definitions"),
    });
    const manifest = readJson(manifestPath);
    delete (manifest as { id?: string }).id;

    const result = validateManifestAndPipeline({
      definitions,
      pipelineName: "character-full",
      manifest,
    });

    expect(result.errors.length).toBeGreaterThan(0);
  });

  it("fails when required manifest inputs are missing", async () => {
    const definitions = await loadJsonDefinitions({
      pipelinesDir: path.join(baseDir, "pipelines/definitions"),
      tasksDir: path.join(baseDir, "tasks/definitions"),
    });
    const manifest = readJson(manifestPath);
    delete (manifest as { textTo3DPreviewTask?: { prompt?: string } }).textTo3DPreviewTask;

    const result = validateManifestAndPipeline({
      definitions,
      pipelineName: "character-full",
      manifest,
    });

    expect(result.errors).toContain(
      "Missing required input 'prompt' for step 'preview' (task 'text-to-3d-preview').",
    );
  });
});
