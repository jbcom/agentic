import fs from "node:fs";
import path from "node:path";
import type { PipelineDefinition, TaskDefinition } from "../schemas/definitions.js";
import { PipelineDefinitionSchema, TaskDefinitionSchema } from "../schemas/definitions.js";
import { readJson } from "../utils/json.js";

/**
 * Parsed pipeline and task definitions loaded from JSON files.
 */
export interface DefinitionBundle {
  pipelines: Map<string, PipelineDefinition>;
  tasks: Map<string, TaskDefinition>;
}

/**
 * Paths to definition directories on disk.
 */
export interface DefinitionLoadOptions {
  /** Directory containing `*.pipeline.json` definitions. */
  pipelinesDir: string;
  /** Directory containing task definition JSON files. */
  tasksDir: string;
}

/**
 * Load and validate pipeline/task definitions from disk.
 *
 * @param options - Directory paths for pipeline and task definitions.
 * @returns Parsed and validated definition bundle.
 */
export async function loadJsonDefinitions(
  options: DefinitionLoadOptions,
): Promise<DefinitionBundle> {
  const pipelines = new Map<string, PipelineDefinition>();
  const tasks = new Map<string, TaskDefinition>();

  const pipelineFiles = fs
    .readdirSync(options.pipelinesDir)
    .filter((file) => file.endsWith(".pipeline.json"));

  for (const file of pipelineFiles) {
    const fullPath = path.join(options.pipelinesDir, file);
    const data = readJson<PipelineDefinition>(fullPath);
    const parsed = PipelineDefinitionSchema.parse(data);
    pipelines.set(parsed.name, parsed);
  }

  const taskFiles = fs.readdirSync(options.tasksDir).filter((file) => file.endsWith(".json"));

  for (const file of taskFiles) {
    const fullPath = path.join(options.tasksDir, file);
    const data = readJson<TaskDefinition>(fullPath);
    if (!("id" in (data as Record<string, unknown>))) continue;
    const parsed = TaskDefinitionSchema.parse(data);
    tasks.set(parsed.id, parsed);
  }

  return { pipelines, tasks };
}
