import { describe, expect, it } from "vitest";
import { buildOpenApiSpec } from "../api/openapi.js";

describe("OpenAPI spec", () => {
  it("includes core routes", () => {
    const spec = buildOpenApiSpec();
    expect(spec.paths["/runs"]).toBeDefined();
    expect(spec.paths["/runs/{runId}"]).toBeDefined();
    expect(spec.paths["/openapi.json"]).toBeDefined();
    expect(spec.paths["/preview"]).toBeDefined();
  });
});
