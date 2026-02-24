import { z } from "zod";

const inputSource = z.enum(["manifest", "step", "literal", "env", "lookup"]);

export const InputBindingSchema = z.object({
  name: z.string(),
  source: inputSource,
  path: z.string().optional(),
  step: z.string().optional(),
  table: z.string().optional(),
  key: z.string().optional(),
  value: z.unknown().optional(),
  default: z.unknown().optional(),
});

export const OutputBindingSchema = z.object({
  name: z.string(),
  responsePath: z.string(),
  artifact: z.string().optional(),
});

export const TaskDefinitionSchema = z.object({
  id: z.string(),
  description: z.string().optional(),
  apiVersion: z.string().optional(),
  endpoint: z.string().optional(),
  method: z.enum(["GET", "POST", "DELETE"]).optional(),
  inputs: z.array(InputBindingSchema).default([]),
  outputs: z.array(OutputBindingSchema).default([]),
  poll: z
    .object({
      strategy: z.enum(["stream", "poll"]).default("stream"),
      path: z.string(),
      intervalMs: z.number().int().positive().optional(),
      timeoutMs: z.number().int().positive().optional(),
    })
    .optional(),
});

export const PipelineStepSchema = z.object({
  id: z.string(),
  task: z.string(),
  dependsOn: z.array(z.string()).optional(),
  forEach: z
    .object({
      source: inputSource,
      path: z.string().optional(),
      step: z.string().optional(),
      table: z.string().optional(),
      key: z.string().optional(),
      as: z.string(),
    })
    .optional(),
  inputs: z.array(InputBindingSchema).optional(),
  stateMapping: z.record(z.string(), z.string()).optional(),
  /** Skip downloading artifacts for this step. Useful for interim pipeline stages. */
  skipArtifacts: z.boolean().optional(),
});

export const PipelineDefinitionSchema = z.object({
  name: z.string(),
  description: z.string().optional(),
  version: z.string().optional(),
  steps: z.array(PipelineStepSchema),
  stateMapping: z.record(z.string(), z.string()).optional(),
});

export type InputBinding = z.infer<typeof InputBindingSchema>;
export type OutputBinding = z.infer<typeof OutputBindingSchema>;
export type TaskDefinition = z.infer<typeof TaskDefinitionSchema>;
export type PipelineStep = z.infer<typeof PipelineStepSchema>;
export type PipelineDefinition = z.infer<typeof PipelineDefinitionSchema>;
