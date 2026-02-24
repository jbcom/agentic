import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { loadJsonDefinitions } from "../core/definitions.js";
import { PipelineRunner } from "../core/runner.js";
import { loadAnimationIds } from "../lookups/animation-ids.js";
import type { TaskResult } from "../meshy/meshy-client.js";

const baseDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../");

class FakeMeshyClient {
  private lastPath = "";

  async post<T>(pathValue: string): Promise<T> {
    this.lastPath = pathValue;
    const response = { result: "task-123" };
    return response as T;
  }

  async streamUntilComplete<T>(): Promise<TaskResult<T>> {
    if (this.lastPath.includes("text-to-image")) {
      return {
        id: "task-123",
        status: "SUCCEEDED",
        progress: 100,
        result: {
          result: {
            image_urls: ["https://example.com/image1.png", "https://example.com/image2.png"],
          },
        } as T,
      };
    }

    return {
      id: "task-123",
      status: "SUCCEEDED",
      progress: 100,
      result: {
        result: {
          model_urls: { glb: "https://example.com/model.glb" },
          texture_urls: [{ base_color: "https://example.com/tex.png" }],
        },
      } as T,
    };
  }
}

describe("PipelineRunner", () => {
  it("executes a pipeline step and maps outputs", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () => ({ ok: true, body: ReadableStream.from(["data"]) })),
    );

    const definitions = await loadJsonDefinitions({
      pipelinesDir: path.join(baseDir, "pipelines/definitions"),
      tasksDir: path.join(baseDir, "tasks/definitions"),
    });

    const runner = new PipelineRunner({
      definitions,
      apiKey: "test",
      client: new FakeMeshyClient() as unknown as never,
      lookups: {
        ANIMATION_IDS: loadAnimationIds(),
      },
      logger: () => { /* noop logger for tests */ },
    });

    const tempDir = path.join(baseDir, ".tmp-test");
    await import("node:fs").then((fs) => fs.mkdirSync(tempDir, { recursive: true }));

    const manifestPath = path.join(tempDir, "manifest.json");
    await import("node:fs").then((fs) =>
      fs.writeFileSync(
        manifestPath,
        JSON.stringify({
          id: "hero",
          name: "Hero",
          type: "prop",
          textToImageTask: { prompt: "Test" },
        }),
      ),
    );

    const result = await runner.run({
      pipelineName: "prop",
      assetDir: tempDir,
    });

    expect(result.tasks?.["multi-image-to-3d"]).toBeDefined();
    vi.unstubAllGlobals();
  });
});
