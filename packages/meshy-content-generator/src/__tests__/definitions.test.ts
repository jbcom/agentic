import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { loadJsonDefinitions } from "../core/definitions.js";

const baseDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../");

describe("definitions loader", () => {
  it("loads pipeline and task definitions", async () => {
    const definitions = await loadJsonDefinitions({
      pipelinesDir: path.join(baseDir, "pipelines/definitions"),
      tasksDir: path.join(baseDir, "tasks/definitions"),
    });

    expect(definitions.pipelines.size).toBeGreaterThan(0);
    expect(definitions.tasks.size).toBeGreaterThan(0);
  });
});
