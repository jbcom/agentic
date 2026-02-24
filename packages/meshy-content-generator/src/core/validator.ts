import type {
  InputBinding,
  PipelineDefinition,
  PipelineStep,
  TaskDefinition,
} from "../schemas/definitions.js";
import type { AssetManifest } from "../schemas/manifest.js";
import { AssetManifestSchema } from "../schemas/manifest.js";
import { getPathValue } from "../utils/object-path.js";
import { applyTemplate } from "../utils/template.js";
import type { DefinitionBundle } from "./definitions.js";

/**
 * Validation errors and warnings produced by the manifest validator.
 */
export interface ValidationResult {
  errors: string[];
  warnings: string[];
}

/**
 * Options for validating a manifest against a pipeline definition.
 */
export interface ValidationOptions {
  definitions: DefinitionBundle;
  pipelineName: string;
  manifest: unknown;
  env?: Record<string, string | undefined>;
  lookups?: Record<string, Record<string, unknown>>;
}

interface ValidationContext {
  manifest: AssetManifest;
  stepResults: Map<
    string,
    { outputs: Record<string, unknown> } | { outputs: Record<string, unknown> }[]
  >;
  iterationVars: Record<string, unknown>;
  env: Record<string, string | undefined>;
  lookups: Record<string, Record<string, unknown>>;
}

/**
 * Validate a manifest and its pipeline/task bindings without executing any Meshy calls.
 */
/**
 * Validate a manifest against the pipeline/task definitions without executing Meshy calls.
 */
export function validateManifestAndPipeline(options: ValidationOptions): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];
  const env = options.env ?? process.env;
  const lookups = options.lookups ?? {};

  const manifestResult = AssetManifestSchema.safeParse(options.manifest);
  if (!manifestResult.success) {
    errors.push(`Manifest schema invalid: ${manifestResult.error.message}`);
    return { errors, warnings };
  }

  const manifest = manifestResult.data;
  const pipeline = options.definitions.pipelines.get(options.pipelineName);
  if (!pipeline) {
    errors.push(`Pipeline definition not found: ${options.pipelineName}`);
    return { errors, warnings };
  }

  const stepIds = new Set<string>();
  for (const step of pipeline.steps) {
    if (stepIds.has(step.id)) {
      errors.push(`Duplicate step id '${step.id}' in pipeline '${pipeline.name}'.`);
    }
    stepIds.add(step.id);
  }

  for (const step of pipeline.steps) {
    for (const dep of step.dependsOn ?? []) {
      if (!stepIds.has(dep)) {
        errors.push(`Step '${step.id}' depends on missing step '${dep}'.`);
      }
    }
  }

  const context: ValidationContext = {
    manifest,
    stepResults: new Map(),
    iterationVars: {},
    env,
    lookups,
  };

  for (const step of pipeline.steps) {
    const task = options.definitions.tasks.get(step.task);
    if (!task) {
      errors.push(`Task definition not found for step '${step.id}': ${step.task}`);
      continue;
    }

    if (step.forEach) {
      const iterable = resolveForEachSource(
        step.forEach.source,
        step.forEach,
        context,
        errors,
        warnings,
      );
      if (Array.isArray(iterable)) {
        iterable.forEach((item, index) => {
          context.iterationVars[step.forEach?.as ?? "item"] = item;
          context.iterationVars[`${step.forEach?.as ?? "item"}_index`] = index;
          validateStepInputs(step, task, pipeline, context, errors, warnings);
        });
      } else {
        errors.push(`forEach source for step '${step.id}' did not resolve to an array.`);
      }
    } else {
      validateStepInputs(step, task, pipeline, context, errors, warnings);
    }
  }

  return { errors, warnings };
}

function validateStepInputs(
  step: PipelineStep,
  taskDef: TaskDefinition,
  pipeline: PipelineDefinition,
  context: ValidationContext,
  errors: string[],
  warnings: string[],
): void {
  const bindings = new Map<string, InputBinding>();
  for (const input of taskDef.inputs) {
    bindings.set(input.name, input);
  }
  for (const input of step.inputs ?? []) {
    bindings.set(input.name, input);
  }

  const templateVars = {
    ...context.iterationVars,
    seed: context.manifest.seed ?? 0,
    assetId: context.manifest.id,
  };

  for (const binding of bindings.values()) {
    const resolved = resolveInput(binding, context, templateVars, pipeline, errors, warnings);
    if (resolved === undefined && binding.default === undefined) {
      if (
        binding.source === "manifest" ||
        binding.source === "env" ||
        binding.source === "lookup"
      ) {
        errors.push(
          `Missing required input '${binding.name}' for step '${step.id}' (task '${taskDef.id}').`,
        );
      } else if (binding.source === "literal") {
        warnings.push(
          `Optional literal input '${binding.name}' has no value for step '${step.id}'.`,
        );
      }
    }
  }
}

function resolveInput(
  binding: InputBinding,
  context: ValidationContext,
  templateVars: Record<string, unknown>,
  pipeline: PipelineDefinition,
  errors: string[],
  warnings: string[],
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
    case "env": {
      const envKey = resolvePath(binding.path);
      if (!envKey) {
        errors.push(`Env binding missing key for '${binding.name}'.`);
        return undefined;
      }
      return context.env[envKey];
    }
    case "manifest": {
      const pathValue = resolvePath(binding.path);
      if (!pathValue) {
        errors.push(`Manifest binding missing path for '${binding.name}'.`);
        return undefined;
      }
      return getPathValue(context.manifest, pathValue);
    }
    case "step":
      if (!binding.step || !binding.path) {
        errors.push(`Step binding missing step/path for '${binding.name}'.`);
        return undefined;
      }
      if (!pipeline.steps.some((step) => step.id === binding.step)) {
        errors.push(
          `Step binding references missing step '${binding.step}' for '${binding.name}'.`,
        );
      } else {
        warnings.push(
          `Step binding '${binding.name}' resolved at runtime from step '${binding.step}'.`,
        );
      }
      return undefined;
    case "lookup": {
      if (!binding.table) {
        errors.push(`Lookup binding missing table for '${binding.name}'.`);
        return undefined;
      }
      const table = context.lookups[binding.table];
      if (!table) {
        errors.push(`Lookup table '${binding.table}' missing for '${binding.name}'.`);
        return undefined;
      }
      const key = applyTemplate(binding.key ?? binding.path ?? "", templateVars);
      if (!key) {
        errors.push(`Lookup binding missing key for '${binding.name}'.`);
        return undefined;
      }
      return table[key];
    }
    default:
      return undefined;
  }
}

function resolveForEachSource(
  source: InputBinding["source"],
  binding: {
    path?: string | undefined;
    step?: string | undefined;
    table?: string | undefined;
    key?: string | undefined;
    as?: string | undefined;
  },
  context: ValidationContext,
  errors: string[],
  warnings: string[],
): unknown {
  const templateVars = {
    ...context.iterationVars,
    seed: context.manifest.seed ?? 0,
    assetId: context.manifest.id,
  };

  switch (source) {
    case "manifest": {
      const pathValue = binding.path ? applyTemplate(binding.path, templateVars) : "";
      return pathValue ? getPathValue(context.manifest, pathValue) : undefined;
    }
    case "step":
      warnings.push(
        `forEach source uses step '${binding.step ?? "unknown"}' and cannot be resolved at validate-time.`,
      );
      return undefined;
    case "literal":
      return binding.path ? applyTemplate(binding.path, templateVars) : binding.path;
    case "env":
      return binding.path ? context.env[applyTemplate(binding.path, templateVars)] : undefined;
    case "lookup": {
      if (!binding.table) {
        errors.push("forEach lookup source missing table.");
        return undefined;
      }
      const table = context.lookups[binding.table];
      if (!table) {
        errors.push(`forEach lookup table '${binding.table}' missing.`);
        return undefined;
      }
      const key = binding.key ?? binding.path;
      if (!key) {
        return Object.values(table);
      }
      return table[applyTemplate(key, templateVars)];
    }
    default:
      return undefined;
  }
}
