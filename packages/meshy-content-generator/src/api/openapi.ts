import {
  extendZodWithOpenApi,
  OpenAPIRegistry,
  OpenApiGeneratorV3,
} from "@asteasolutions/zod-to-openapi";
import { z } from "zod";
import { AssetManifestSchema } from "../schemas/manifest.js";

extendZodWithOpenApi(z);

export const RunRequestSchema = z.object({
  pipelineName: z.string(),
  assetDir: z.string(),
  step: z.string().optional(),
  wait: z.boolean().optional(),
});

export const RunStatusSchema = z.enum(["QUEUED", "RUNNING", "SUCCEEDED", "FAILED"]);

export const RunResponseSchema = z.object({
  runId: z.string(),
  status: RunStatusSchema,
  manifest: AssetManifestSchema.optional(),
  error: z.string().optional(),
});

export const RunRecordSchema = z.object({
  runId: z.string(),
  status: RunStatusSchema,
  manifest: AssetManifestSchema.optional(),
  error: z.string().optional(),
});

/**
 * Build the OpenAPI document describing the Meshy content generation API.
 */
export function buildOpenApiSpec() {
  const registry = new OpenAPIRegistry();

  registry.register("RunRequest", RunRequestSchema);
  registry.register("RunResponse", RunResponseSchema);
  registry.register("RunRecord", RunRecordSchema);

  registry.registerPath({
    method: "post",
    path: "/runs",
    description: "Start a Meshy pipeline run.",
    request: {
      body: {
        content: {
          "application/json": { schema: RunRequestSchema },
        },
      },
    },
    responses: {
      200: {
        description: "Run accepted",
        content: {
          "application/json": { schema: RunResponseSchema },
        },
      },
    },
  });

  registry.registerPath({
    method: "get",
    path: "/runs/{runId}",
    description: "Fetch run status and results.",
    request: {
      params: z.object({ runId: z.string() }),
    },
    responses: {
      200: {
        description: "Run status",
        content: {
          "application/json": { schema: RunRecordSchema },
        },
      },
      404: {
        description: "Run not found",
      },
    },
  });

  registry.registerPath({
    method: "get",
    path: "/openapi.json",
    description: "OpenAPI spec for the Meshy content generation API.",
    responses: {
      200: {
        description: "OpenAPI spec",
      },
    },
  });

  registry.registerPath({
    method: "get",
    path: "/preview",
    description: "Preview a generated GLB using a lightweight viewer.",
    request: {
      query: z.object({
        assetDir: z.string(),
        file: z.string(),
      }),
    },
    responses: {
      200: {
        description: "HTML preview",
      },
    },
  });

  registry.registerPath({
    method: "get",
    path: "/file",
    description: "Serve a file within an asset directory for previewing.",
    request: {
      query: z.object({
        assetDir: z.string(),
        file: z.string(),
      }),
    },
    responses: {
      200: {
        description: "Binary asset file",
      },
      404: {
        description: "File not found",
      },
    },
  });

  const generator = new OpenApiGeneratorV3(registry.definitions);

  return generator.generateDocument({
    openapi: "3.0.3",
    info: {
      title: "Meshy Content Generator API",
      version: "0.1.0",
      description: "API for running declarative Meshy content pipelines.",
    },
  });
}
