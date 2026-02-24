import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { loadJsonDefinitions } from "../core/definitions.js";
import { PipelineRunner } from "../core/runner.js";
import { loadAnimationIds } from "../lookups/animation-ids.js";
import { createPolly } from "./polly.js";

const baseDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../");
const recordingsDir = path.join(baseDir, "test/__recordings__");
const recordingFile = path.join(recordingsDir, "meshy-character-full.json");
const mode = process.env.POLLY_MODE ?? "replay";

describe("Full Meshy pipeline (VCR)", () => {
  const shouldRun = mode === "record" || fs.existsSync(recordingFile);

  const testFn = shouldRun ? it : it.skip;

  testFn(
    "runs text → image → preview → refine → rigging → animation",
    async () => {
      if (process.env.POLLY_MODE === "record" && !process.env.MESHY_API_KEY) {
        throw new Error("MESHY_API_KEY is required for recording.");
      }

      const polly = createPolly("meshy-character-full");

      const fixtureDir = path.join(baseDir, "test/fixtures/character-full");
      const tempDir = path.join(baseDir, ".tmp-full-pipeline");
      fs.rmSync(tempDir, { recursive: true, force: true });
      fs.mkdirSync(tempDir, { recursive: true });
      fs.copyFileSync(path.join(fixtureDir, "manifest.json"), path.join(tempDir, "manifest.json"));

      const definitions = await loadJsonDefinitions({
        pipelinesDir: path.join(baseDir, "pipelines/definitions"),
        tasksDir: path.join(baseDir, "tasks/definitions"),
      });

      const runner = new PipelineRunner({
        definitions,
        apiKey: process.env.MESHY_API_KEY ?? "",
        lookups: {
          ANIMATION_IDS: loadAnimationIds(),
        },
      });

      const manifest = await runner.run({
        pipelineName: "character-full",
        assetDir: tempDir,
      });

      expect(manifest.tasks?.["text-to-image"]).toBeDefined();
      expect(manifest.tasks?.["text-to-3d-preview"]).toBeDefined();
      expect(manifest.tasks?.["text-to-3d-refine"]).toBeDefined();
      expect(manifest.tasks?.rigging).toBeDefined();
      expect(manifest.tasks?.animation).toBeDefined();

      await polly.stop();
      fs.rmSync(tempDir, { recursive: true, force: true });
    },
    1_800_000,
  );
});
