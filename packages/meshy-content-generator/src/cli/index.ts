import path from "node:path";
import { Command } from "commander";
import dotenv from "dotenv";
import { loadJsonDefinitions } from "../core/definitions.js";
import { PipelineRunner } from "../core/runner.js";
import { validateManifestAndPipeline } from "../core/validator.js";
import { loadAnimationIds } from "../lookups/animation-ids.js";
import { readJson } from "../utils/json.js";

const program = new Command();

dotenv.config();

program
  .name("content-gen")
  .description("Declarative content generation pipelines")
  .version("0.1.0");

function resolveDir(input: string | undefined, fallback: string): string {
  if (input) return path.resolve(process.cwd(), input);
  return path.resolve(process.cwd(), fallback);
}

async function buildRunner(options: {
  pipelines: string;
  tasks: string;
  meshyApiKey?: string;
}): Promise<PipelineRunner> {
  const definitions = await loadJsonDefinitions({
    pipelinesDir: options.pipelines,
    tasksDir: options.tasks,
  });

  const apiKey = options.meshyApiKey ?? process.env.MESHY_API_KEY;
  if (!apiKey) {
    throw new Error("MESHY_API_KEY is required.");
  }

  return new PipelineRunner({
    definitions,
    apiKey,
    lookups: {
      ANIMATION_IDS: loadAnimationIds(),
    },
  });
}

program
  .command("list")
  .description("List available pipelines and tasks")
  .option("--pipelines <dir>", "Pipelines definitions directory", "pipelines/definitions")
  .option("--tasks <dir>", "Task definitions directory", "tasks/definitions")
  .action(async (opts) => {
    const pipelinesDir = resolveDir(opts.pipelines, "pipelines/definitions");
    const tasksDir = resolveDir(opts.tasks, "tasks/definitions");
    const definitions = await loadJsonDefinitions({ pipelinesDir, tasksDir });

    console.log("\nPipelines:");
    const pipelineList = Array.from(definitions.pipelines.values()).sort((a, b) =>
      a.name.localeCompare(b.name),
    );
    for (const pipeline of pipelineList) {
      console.log(`- ${pipeline.name}`);
    }

    console.log("\nTasks:");
    const taskList = Array.from(definitions.tasks.values()).sort((a, b) =>
      a.id.localeCompare(b.id),
    );
    for (const task of taskList) {
      console.log(`- ${task.id}`);
    }
  });

program
  .command("validate")
  .description("Validate a manifest against pipeline/task definitions")
  .argument("<assetDir>", "Asset directory containing manifest.json")
  .option("--pipelines <dir>", "Pipelines definitions directory", "pipelines/definitions")
  .option("--tasks <dir>", "Task definitions directory", "tasks/definitions")
  .option("--pipeline <name>", "Pipeline name to validate")
  .action(async (assetDir, opts) => {
    const pipelinesDir = resolveDir(opts.pipelines, "pipelines/definitions");
    const tasksDir = resolveDir(opts.tasks, "tasks/definitions");
    const definitions = await loadJsonDefinitions({ pipelinesDir, tasksDir });

    if (opts.pipeline && !definitions.pipelines.has(opts.pipeline)) {
      throw new Error(`Pipeline not found: ${opts.pipeline}`);
    }

    const manifestPath = path.join(path.resolve(process.cwd(), assetDir), "manifest.json");
    const exists = await import("node:fs").then((fs) => fs.existsSync(manifestPath));
    if (!exists) {
      throw new Error(`manifest.json not found at ${manifestPath}`);
    }

    const manifest = readJson(manifestPath);
    const pipelineNames = Array.from(definitions.pipelines.keys());
    const pipelineName = opts.pipeline ?? pipelineNames[0];
    if (!pipelineName) {
      throw new Error("No pipeline definitions found to validate.");
    }
    if (!opts.pipeline && pipelineNames.length > 1) {
      console.warn(
        `No pipeline specified; defaulting to '${pipelineName}'. Use --pipeline to choose another.`,
      );
    }

    const result = validateManifestAndPipeline({
      definitions,
      pipelineName,
      manifest,
      lookups: {
        ANIMATION_IDS: loadAnimationIds(),
      },
    });

    if (result.warnings.length > 0) {
      console.warn("Validation warnings:");
      for (const warning of result.warnings) {
        console.warn(`- ${warning}`);
      }
    }

    if (result.errors.length > 0) {
      const message = ["Validation failed:", ...result.errors.map((error) => `- ${error}`)].join(
        "\n",
      );
      throw new Error(message);
    }

    console.log("Manifest + definitions validated.");
  });

program
  .command("run")
  .description("Run a pipeline against an asset manifest")
  .argument("<pipeline>", "Pipeline name")
  .argument("<assetDir>", "Asset directory containing manifest.json")
  .option("--pipelines <dir>", "Pipelines definitions directory", "pipelines/definitions")
  .option("--tasks <dir>", "Task definitions directory", "tasks/definitions")
  .option("--step <id>", "Optional step id to run")
  .option("--meshy-api-key <key>", "Meshy API key (overrides env)")
  .action(async (pipelineName, assetDir, opts) => {
    const pipelinesDir = resolveDir(opts.pipelines, "pipelines/definitions");
    const tasksDir = resolveDir(opts.tasks, "tasks/definitions");

    const runner = await buildRunner({
      pipelines: pipelinesDir,
      tasks: tasksDir,
      meshyApiKey: opts.meshyApiKey,
    });

    await runner.run({
      pipelineName,
      assetDir: path.resolve(process.cwd(), assetDir),
      step: opts.step,
    });
  });

program.parse();
