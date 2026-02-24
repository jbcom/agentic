import { describe, expect, it } from "vitest";
import { buildOpenApiSpec } from "../api/openapi.js";

function getSchema(spec: ReturnType<typeof buildOpenApiSpec>, name: string) {
  const schemas = spec.components?.schemas ?? {};
  return schemas[name] as {
    type?: string;
    required?: string[];
    properties?: Record<string, unknown>;
  };
}

describe("OpenAPI contracts", () => {
  it("exposes RunRequest with required fields", () => {
    const spec = buildOpenApiSpec();
    const schema = getSchema(spec, "RunRequest");
    expect(schema).toBeDefined();
    expect(schema.type).toBe("object");
    expect(schema.required).toEqual(["pipelineName", "assetDir"]);
  });

  it("exposes RunResponse with status enum", () => {
    const spec = buildOpenApiSpec();
    const schema = getSchema(spec, "RunResponse");
    expect(schema).toBeDefined();
    const status = schema.properties?.status as { enum?: string[] } | undefined;
    expect(status?.enum).toEqual(["QUEUED", "RUNNING", "SUCCEEDED", "FAILED"]);
  });
});
