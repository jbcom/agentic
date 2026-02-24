import fs from "node:fs";
import path from "node:path";
import { Readable } from "node:stream";
import type { ReadableStream as WebReadableStream } from "node:stream/web";
import seedrandom from "seedrandom";
import type { TaskResult } from "../meshy/meshy-client.js";
import { MeshyClient } from "../meshy/meshy-client.js";
import type {
  InputBinding,
  PipelineDefinition,
  PipelineStep,
  TaskDefinition,
} from "../schemas/definitions.js";
import type { AssetManifest, TaskState } from "../schemas/manifest.js";
import { AssetManifestSchema } from "../schemas/manifest.js";
import { readJson, writeJson } from "../utils/json.js";
import { getPathValue, setPathValue } from "../utils/object-path.js";
import { applyTemplate } from "../utils/template.js";
import type { DefinitionBundle } from "./definitions.js";

export interface PipelineRunnerOptions {
  /** Loaded pipeline/task definitions. */
  definitions: DefinitionBundle;
  /** Meshy API key used to authenticate requests. */
  apiKey: string;
  /** Optional Meshy client override (useful for testing). */
  client?: MeshyClient;
  /** Lookup tables for binding inputs by key. */
  lookups?: Record<string, Record<string, unknown>>;
  /** Optional logger to capture pipeline progress. */
  logger?: (message: string) => void;
}

export interface RunOptions {
  /** Pipeline definition name. */
  pipelineName: string;
  /** Asset directory containing the manifest.json file. */
  assetDir: string;
  /** Optional step id to run within the pipeline. */
  step?: string;
}

interface StepResult {
  taskId?: string;
  status: string;
  outputs: Record<string, unknown>;
  artifacts: Record<string, string>;
}

interface ExecutionContext {
  manifest: AssetManifest;
  assetDir: string;
  stepResults: Map<string, StepResult | StepResult[]>;
  iterationVars: Record<string, unknown>;
  seed: number;
}

/**
 * Executes declarative pipelines against asset manifests.
 */
export class PipelineRunner {
  private readonly definitions: DefinitionBundle;
  private readonly client: MeshyClient;
  private readonly lookups: Record<string, Record<string, unknown>>;
  private readonly logger: (message: string) => void;

  constructor(options: PipelineRunnerOptions) {
    this.definitions = options.definitions;
    this.client = options.client ?? new MeshyClient({ apiKey: options.apiKey });
    this.lookups = options.lookups ?? {};
    this.logger = options.logger ?? console.log;
  }

  /**
   * Run a pipeline against the manifest in the provided asset directory.
   *
   * @param options - Pipeline execution options.
   * @returns Updated manifest with task state.
   */
  async run(options: RunOptions): Promise<AssetManifest> {
    const pipeline = this.getPipeline(options.pipelineName);
    const manifestPath = path.join(options.assetDir, "manifest.json");
    if (!fs.existsSync(manifestPath)) {
      throw new Error(`Manifest not found: ${manifestPath}`);
    }

    const manifest = AssetManifestSchema.parse(readJson<AssetManifest>(manifestPath));
    const seeded = this.ensureSeed(manifest, manifestPath);

    const context: ExecutionContext = {
      manifest: seeded,
      assetDir: options.assetDir,
      stepResults: new Map(),
      iterationVars: {},
      seed: seeded.seed ?? 0,
    };

    this.loadExistingResults(pipeline, context);

    const stepsToRun = options.step
      ? pipeline.steps.filter((step) => step.id === options.step)
      : pipeline.steps;

    for (const step of stepsToRun) {
      await this.executeStep(step, pipeline, context);
      writeJson(manifestPath, context.manifest);
    }

    return context.manifest;
  }

  /**
   * Resolve a pipeline definition by name.
   */
  private getPipeline(name: string): PipelineDefinition {
    const pipeline = this.definitions.pipelines.get(name);
    if (!pipeline) throw new Error(`Pipeline definition not found: ${name}`);
    return pipeline;
  }

  /**
   * Resolve a task definition by id.
   */
  private getTask(id: string): TaskDefinition {
    const task = this.definitions.tasks.get(id);
    if (!task) throw new Error(`Task definition not found: ${id}`);
    return task;
  }

  /**
   * Ensure the manifest has a deterministic seed, generating one if missing.
   */
  private ensureSeed(manifest: AssetManifest, manifestPath: string): AssetManifest {
    if (manifest.seed !== undefined) return manifest;
    const rng = seedrandom();
    const seed = rng.int32() >>> 0;
    const updated = { ...manifest, seed };
    writeJson(manifestPath, updated);
    return updated;
  }

  /**
   * Load existing task state from the manifest into the execution context.
   */
  private loadExistingResults(pipeline: PipelineDefinition, context: ExecutionContext): void {
    const tasks = context.manifest.tasks ?? {};
    for (const step of pipeline.steps) {
      const state = tasks[step.task];
      if (!state) continue;
      const result: StepResult = {
        status: state.status ?? "UNKNOWN",
        outputs: state.outputs ?? {},
        artifacts: state.artifacts ?? {},
      };
      if (state.taskId) {
        result.taskId = state.taskId;
      }
      context.stepResults.set(step.id, result);
    }
  }

  /**
   * Execute a pipeline step (including dependency checks and forEach handling).
   */
  private async executeStep(
    step: PipelineStep,
    pipeline: PipelineDefinition,
    context: ExecutionContext,
  ): Promise<void> {
    for (const dep of step.dependsOn ?? []) {
      const existing = context.stepResults.get(dep);
      if (!existing) {
        throw new Error(`Dependency '${dep}' not completed for step '${step.id}'`);
      }
    }

    if (step.forEach) {
      await this.executeForEach(step, pipeline, context);
      return;
    }

    const stepResult = await this.executeTask(step, context);
    context.stepResults.set(step.id, stepResult);
    this.applyStateMappings(pipeline, step, context, stepResult);
  }

  /**
   * Execute a step multiple times for each resolved item.
   */
  private async executeForEach(
    step: PipelineStep,
    pipeline: PipelineDefinition,
    context: ExecutionContext,
  ): Promise<void> {
    const forEach = step.forEach;
    if (!forEach) return;

    const iterable = this.resolveForEachSource(forEach.source, forEach, context);
    if (!Array.isArray(iterable)) {
      throw new Error(`forEach source for step '${step.id}' did not resolve to an array.`);
    }

    const results: StepResult[] = [];
    let index = 0;
    for (const item of iterable) {
      context.iterationVars[forEach.as] = item;
      context.iterationVars[`${forEach.as}_index`] = index;
      const result = await this.executeTask(step, context);
      results.push(result);
      index += 1;
    }

    context.stepResults.set(step.id, results);
    if (results.length > 0) {
      const latest = results[results.length - 1];
      if (latest) {
        this.applyStateMappings(pipeline, step, context, latest);
      }
    }
  }

  /**
   * Resolve a forEach source array from manifest, step output, or literal input.
   */
  private resolveForEachSource(
    source: InputBinding["source"],
    binding: {
      path?: string | undefined;
      step?: string | undefined;
      table?: string | undefined;
      key?: string | undefined;
    },
    context: ExecutionContext,
  ): unknown {
    switch (source) {
      case "manifest":
        return binding.path ? getPathValue(context.manifest, binding.path) : undefined;
      case "step":
        if (!binding.step || !binding.path) return undefined;
        return this.resolveStepOutput(binding.step, binding.path, context);
      case "literal":
        return binding.path;
      case "env":
        return binding.path ? process.env[binding.path] : undefined;
      case "lookup":
        if (!binding.table) return undefined;
        if (binding.key) {
          const resolvedKey = applyTemplate(binding.key, {
            ...context.iterationVars,
            seed: context.seed,
            assetId: context.manifest.id,
          });
          return this.lookups[binding.table]?.[resolvedKey];
        }
        return Object.values(this.lookups[binding.table] ?? {});
      default:
        return undefined;
    }
  }

  /**
   * Execute a single task using the configured provider.
   * Skips already-succeeded tasks and only downloads missing artifacts.
   */
  private async executeTask(step: PipelineStep, context: ExecutionContext): Promise<StepResult> {
    const taskDef = this.getTask(step.task);
    const shouldDownloadArtifacts = !step.skipArtifacts;

    // Check if task already succeeded - skip re-running, just ensure artifacts
    const existingResult = context.stepResults.get(step.id);
    const existing = Array.isArray(existingResult) ? existingResult[0] : existingResult;
    if (existing && existing.status === "SUCCEEDED" && existing.taskId) {
      this.logger(
        `\n▶ Step ${step.id} (${taskDef.id}) - already SUCCEEDED${shouldDownloadArtifacts ? ", checking artifacts" : ", skipping artifacts"}`,
      );

      // Check for missing artifacts and download them (unless skipArtifacts is set)
      if (shouldDownloadArtifacts) {
        const missingArtifacts = await this.downloadMissingArtifacts(taskDef, existing, context);
        if (Object.keys(missingArtifacts).length > 0) {
          existing.artifacts = { ...existing.artifacts, ...missingArtifacts };
          this.updateTaskArtifacts(taskDef, existing.artifacts, context);
        }
      }

      return existing;
    }

    const inputs = this.resolveInputs(taskDef, step, context);

    this.logger(`\n▶ Step ${step.id} (${taskDef.id})`);
    const result = await this.runMeshyTask(taskDef, inputs);

    const outputs = {
      ...this.mapOutputs(taskDef, result),
      taskId: result.id,
    };

    // Download artifacts unless skipArtifacts is set on the step
    const artifacts = shouldDownloadArtifacts
      ? await this.downloadArtifacts(taskDef, outputs, context)
      : {};

    this.persistTaskState(taskDef, result, outputs, artifacts, context);

    return {
      taskId: result.id,
      status: result.status,
      outputs,
      artifacts,
    };
  }

  /**
   * Download artifacts that are missing from an already-succeeded task.
   * Fetches task details from Meshy API if URLs are not cached in outputs.
   */
  private async downloadMissingArtifacts(
    taskDef: TaskDefinition,
    existing: StepResult,
    context: ExecutionContext,
  ): Promise<Record<string, string>> {
    const missingArtifacts: Record<string, string> = {};

    // Check which outputs need downloading
    const outputsNeedingDownload: Array<{
      output: (typeof taskDef.outputs)[0];
      filename: string;
      targetPath: string;
    }> = [];

    for (const output of taskDef.outputs) {
      if (!output.artifact) continue;

      const filename = applyTemplate(output.artifact, {
        ...context.iterationVars,
        seed: context.seed,
        assetId: context.manifest.id,
      });
      const targetPath = path.join(context.assetDir, filename);

      if (fs.existsSync(targetPath)) {
        this.logger(`  ✓ ${filename} already exists`);
        continue;
      }

      outputsNeedingDownload.push({ output, filename, targetPath });
    }

    if (outputsNeedingDownload.length === 0) {
      return missingArtifacts;
    }

    // Fetch task details from Meshy API to get download URLs
    let outputs = existing.outputs;
    if (existing.taskId) {
      try {
        const apiVersion = taskDef.apiVersion ?? "v1";
        const endpoint = taskDef.endpoint ?? taskDef.id;
        const fetchPath = `/${apiVersion}/${endpoint}/${existing.taskId}`;
        const taskDetails = await this.client.get<Record<string, unknown>>(fetchPath);

        // Map the response to outputs
        // Some APIs return { result: { url: ... } }, others return { model_urls: { glb: ... } }
        // Wrap in { result: ... } only if the response doesn't already have a result property
        // AND the responsePath starts with "result."
        const hasResultProperty =
          taskDetails && typeof taskDetails === "object" && "result" in taskDetails;
        outputs = {};
        for (const output of taskDef.outputs) {
          let source: Record<string, unknown>;
          if (output.responsePath.startsWith("result.") && !hasResultProperty) {
            source = { result: taskDetails };
          } else {
            source = taskDetails as Record<string, unknown>;
          }
          const value = getPathValue(source, output.responsePath);
          outputs[output.name] = value;
        }
      } catch (err) {
        this.logger(`  ⚠ Failed to fetch task details: ${err}`);
        return missingArtifacts;
      }
    }

    // Download the files
    for (const { output, filename, targetPath } of outputsNeedingDownload) {
      const url = outputs[output.name];
      if (typeof url === "string" && url.trim()) {
        this.logger(`  ↓ Downloading ${filename}...`);
        try {
          await this.downloadFile(url, targetPath);
          missingArtifacts[output.name] = filename;
          this.logger(`  ✓ Downloaded ${filename}`);
        } catch (err) {
          this.logger(`  ✗ Failed to download ${filename}: ${err}`);
        }
      } else {
        this.logger(`  ⚠ No URL for ${output.name}`);
      }
    }

    return missingArtifacts;
  }

  /**
   * Update only the artifacts in manifest task state without overwriting other fields.
   */
  private updateTaskArtifacts(
    taskDef: TaskDefinition,
    artifacts: Record<string, string>,
    context: ExecutionContext,
  ): void {
    const tasks = context.manifest.tasks ?? {};
    const state = tasks[taskDef.id];
    if (state) {
      state.artifacts = { ...state.artifacts, ...artifacts };
    }
  }

  /**
   * Resolve task inputs from manifest, step outputs, env vars, or lookups.
   */
  private resolveInputs(
    taskDef: TaskDefinition,
    step: PipelineStep,
    context: ExecutionContext,
  ): Record<string, unknown> {
    const bindings = new Map<string, InputBinding>();
    for (const input of taskDef.inputs) {
      bindings.set(input.name, input);
    }
    for (const input of step.inputs ?? []) {
      bindings.set(input.name, input);
    }

    const resolved: Record<string, unknown> = {};
    const templateVars = {
      ...context.iterationVars,
      seed: context.seed,
      assetId: context.manifest.id,
    };

    for (const binding of bindings.values()) {
      const value = this.resolveInput(binding, context, templateVars);
      if (value !== undefined) {
        resolved[binding.name] = value;
      } else if (binding.default !== undefined) {
        resolved[binding.name] = binding.default;
      }
    }

    return resolved;
  }

  /**
   * Resolve one input binding into a concrete value.
   */
  private resolveInput(
    binding: InputBinding,
    context: ExecutionContext,
    templateVars: Record<string, unknown>,
  ): unknown {
    const resolvePath = (raw?: string): string | undefined => {
      if (!raw) return undefined;
      return applyTemplate(raw, templateVars);
    };

    switch (binding.source) {
      case "literal":
        if (typeof binding.value === "string") {
          return applyTemplate(binding.value, templateVars);
        }
        return binding.value;
      case "env":
        return binding.path ? process.env[resolvePath(binding.path) ?? ""] : undefined;
      case "manifest":
        return binding.path
          ? getPathValue(context.manifest, resolvePath(binding.path) ?? "")
          : undefined;
      case "step":
        if (!binding.step || !binding.path) return undefined;
        return this.resolveStepOutput(binding.step, resolvePath(binding.path) ?? "", context);
      case "lookup":
        if (!binding.table) return undefined;
        return this.lookups[binding.table]?.[applyTemplate(binding.key ?? "", templateVars)];
      default:
        return undefined;
    }
  }

  /**
   * Read a value from a previous step output by path.
   */
  private resolveStepOutput(stepId: string, pathValue: string, context: ExecutionContext): unknown {
    const result = context.stepResults.get(stepId);
    if (!result) return undefined;
    const latest = Array.isArray(result) ? result[result.length - 1] : result;
    if (!latest) return undefined;
    return getPathValue(latest.outputs, pathValue);
  }

  /**
   * Map provider response into named outputs based on task definition.
   */
  private mapOutputs(
    taskDef: TaskDefinition,
    result: TaskResult<Record<string, unknown>>,
  ): Record<string, unknown> {
    const outputs: Record<string, unknown> = {};
    const source = (result.result ?? (result as unknown as Record<string, unknown>)) as Record<
      string,
      unknown
    >;
    for (const output of taskDef.outputs) {
      const value = getPathValue(source, output.responsePath);
      outputs[output.name] = value;
    }
    return outputs;
  }

  /**
   * Download artifact URLs into the asset directory.
   */
  private async downloadArtifacts(
    taskDef: TaskDefinition,
    outputs: Record<string, unknown>,
    context: ExecutionContext,
  ): Promise<Record<string, string>> {
    const artifacts: Record<string, string> = {};

    for (const output of taskDef.outputs) {
      if (!output.artifact) continue;
      const value = outputs[output.name];
      if (typeof value === "string") {
        if (!value.trim()) continue;
        const filename = applyTemplate(output.artifact, {
          ...context.iterationVars,
          seed: context.seed,
          assetId: context.manifest.id,
        });
        const targetPath = path.join(context.assetDir, filename);
        await this.downloadFile(value, targetPath);
        artifacts[output.name] = filename;
        continue;
      }

      if (Array.isArray(value)) {
        let index = 0;
        for (const item of value) {
          if (typeof item !== "string" || !item.trim()) {
            index += 1;
            continue;
          }
          const filename = applyTemplate(output.artifact, {
            ...context.iterationVars,
            seed: context.seed,
            assetId: context.manifest.id,
            index,
          });
          const targetPath = path.join(context.assetDir, filename);
          await this.downloadFile(item, targetPath);
          artifacts[`${output.name}.${index}`] = filename;
          index += 1;
        }
      }
    }

    return artifacts;
  }

  /**
   * Download a single file from URL to a local path.
   */
  private async downloadFile(url: string, targetPath: string): Promise<void> {
    if (!url || !url.trim()) return;
    const response = await fetch(url);
    if (!response.ok || !response.body) {
      throw new Error(`Failed to download artifact: ${url}`);
    }

    await fs.promises.mkdir(path.dirname(targetPath), { recursive: true });
    const fileStream = fs.createWriteStream(targetPath);
    const nodeStream = Readable.fromWeb(response.body as unknown as WebReadableStream);
    await new Promise<void>((resolve, reject) => {
      nodeStream
        .pipe(fileStream)
        .on("finish", () => resolve())
        .on("error", (error: Error) => reject(error));
    });
  }

  /**
   * Persist task outputs into the manifest's task state.
   */
  private persistTaskState(
    taskDef: TaskDefinition,
    result: TaskResult<Record<string, unknown>>,
    outputs: Record<string, unknown>,
    artifacts: Record<string, string>,
    context: ExecutionContext,
  ): void {
    const tasks = context.manifest.tasks ?? {};
    const state: TaskState = {
      taskId: result.id,
      status: result.status,
      outputs,
      artifacts,
    };
    tasks[taskDef.id] = state;
    context.manifest.tasks = tasks;
  }

  /**
   * Apply output values into manifest paths using mapping rules.
   */
  private applyStateMappings(
    pipeline: PipelineDefinition,
    step: PipelineStep,
    context: ExecutionContext,
    stepResult: StepResult,
  ): void {
    const mapping = {
      ...(pipeline.stateMapping ?? {}),
      ...(step.stateMapping ?? {}),
    };

    for (const [manifestPath, outputPath] of Object.entries(mapping)) {
      const value = getPathValue(stepResult.outputs, outputPath);
      if (value !== undefined) {
        setPathValue(context.manifest as Record<string, unknown>, manifestPath, value);
      }
    }
  }

  private async runMeshyTask(
    taskDef: TaskDefinition,
    inputs: Record<string, unknown>,
  ): Promise<TaskResult<Record<string, unknown>>> {
    const apiVersion = taskDef.apiVersion ?? "v1";
    const endpoint = taskDef.endpoint ?? taskDef.id;
    const method = taskDef.method ?? "POST";

    if (method !== "POST") {
      throw new Error(`Unsupported Meshy method '${method}' for task '${taskDef.id}'.`);
    }

    const createPath = `/${apiVersion}/${endpoint}`;
    const payload = Object.fromEntries(
      Object.entries(inputs).filter(([, value]) => value !== undefined && value !== ""),
    );

    const createResponse = await this.client.post<{ result?: string; id?: string }>(
      createPath,
      payload,
    );
    const taskId = createResponse.result ?? createResponse.id;
    if (!taskId) {
      throw new Error(
        `Meshy task creation failed for ${taskDef.id}: ${JSON.stringify(createResponse)}`,
      );
    }

    const streamPath = `/${apiVersion}/${endpoint}/${taskId}/stream`;
    const result = await this.client.streamUntilComplete<Record<string, unknown>>(streamPath);

    if (result.result) {
      return { ...result, id: taskId };
    }

    const fetchPath = `/${apiVersion}/${endpoint}/${taskId}`;
    const fetched = await this.client.get<Record<string, unknown>>(fetchPath);
    const fetchedResult =
      typeof fetched === "object" && fetched !== null && "result" in fetched
        ? (fetched as { result?: Record<string, unknown> }).result
        : fetched;

    const combined: TaskResult<Record<string, unknown>> = {
      ...result,
      ...(typeof fetched === "object" ? fetched : {}),
      id: taskId,
    };
    const finalResult = fetchedResult ?? result.result;
    if (typeof finalResult !== "undefined") {
      combined.result = finalResult;
    }

    return combined;
  }
}
