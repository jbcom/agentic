import { z } from "zod";

export const TaskStateSchema = z
  .object({
    taskId: z.string().optional(),
    status: z.enum(["PENDING", "IN_PROGRESS", "SUCCEEDED", "FAILED", "CANCELED"]).optional(),
    outputs: z.record(z.string(), z.unknown()).optional(),
    artifacts: z.record(z.string(), z.string()).optional(),
  })
  .passthrough();

export const AssetManifestSchema = z
  .object({
    id: z.string(),
    name: z.string(),
    type: z.string(),
    seed: z.number().int().nonnegative().optional(),
    tasks: z.record(z.string(), TaskStateSchema).optional(),
  })
  .passthrough();

export type AssetManifest = z.infer<typeof AssetManifestSchema>;
export type TaskState = z.infer<typeof TaskStateSchema>;
