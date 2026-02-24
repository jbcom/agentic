import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import scalarFastifyApiReference from "@scalar/fastify-api-reference";
import Fastify from "fastify";
import { loadJsonDefinitions } from "../core/definitions.js";
import { PipelineRunner } from "../core/runner.js";
import { loadAnimationIds } from "../lookups/animation-ids.js";
import { buildOpenApiSpec, RunRequestSchema } from "./openapi.js";

/**
 * Build the Fastify server for the content generator API + preview.
 *
 * @param options - Server options (defaults to process.cwd()).
 */
export function buildServer(options?: { cwd?: string }) {
  const app = Fastify({ logger: true });
  const runs = new Map<
    string,
    { status: "QUEUED" | "RUNNING" | "SUCCEEDED" | "FAILED"; manifest?: unknown; error?: string }
  >();

  const cwd = options?.cwd ?? process.cwd();

  const openapi = buildOpenApiSpec();

  app.get("/openapi.json", async (_request, reply) => {
    reply.send(openapi);
  });

  app.register(scalarFastifyApiReference, {
    routePrefix: "/api",
    logLevel: "silent",
    configuration: {
      spec: {
        content: openapi,
      },
      theme: "purple",
      customCss: `
        :root {
          --scalar-color-1: #f3f4f6;
          --scalar-color-2: #d9dbe1;
          --scalar-color-3: #b3b8c5;
          --scalar-color-accent: #8b5cf6;
          --scalar-color-accent-2: #a78bfa;
          --scalar-background-1: #0b0b12;
          --scalar-background-2: #12121c;
          --scalar-background-3: #171726;
          font-family: "Space Grotesk", "Inter", system-ui, -apple-system, sans-serif;
        }
      `,
    },
  });

  app.post("/runs", async (request, reply) => {
    const parseResult = RunRequestSchema.safeParse(request.body);
    if (!parseResult.success) {
      reply.status(400).send({ error: parseResult.error.message });
      return;
    }

    const { pipelineName, assetDir, step, wait } = parseResult.data;
    const runId = crypto.randomUUID();
    runs.set(runId, { status: "QUEUED" });

    const absoluteAssetDir = path.resolve(cwd, assetDir);
    if (!absoluteAssetDir.startsWith(cwd + path.sep)) {
      reply
        .status(400)
        .send({ error: "Asset directory must be within the current working directory." });
      return;
    }

    const apiKey = process.env.MESHY_API_KEY;
    if (!apiKey) {
      reply.status(400).send({ error: "MESHY_API_KEY is required." });
      return;
    }

    const runner = new PipelineRunner({
      definitions: await loadJsonDefinitions({
        pipelinesDir: path.resolve(cwd, "pipelines/definitions"),
        tasksDir: path.resolve(cwd, "tasks/definitions"),
      }),
      apiKey,
      lookups: {
        ANIMATION_IDS: loadAnimationIds(),
      },
    });

    const runPromise = (async () => {
      runs.set(runId, { status: "RUNNING" });
      try {
        const manifest = await runner.run({
          pipelineName,
          assetDir: absoluteAssetDir,
          ...(step !== undefined ? { step } : {}),
        });
        runs.set(runId, { status: "SUCCEEDED", manifest });
      } catch (error) {
        runs.set(runId, {
          status: "FAILED",
          error: error instanceof Error ? error.message : String(error),
        });
      }
    })();

    if (wait) {
      await runPromise;
    }

    const record = runs.get(runId);
    reply.send({
      runId,
      status: record?.status ?? "QUEUED",
      manifest: record?.manifest,
      error: record?.error,
    });
  });

  app.get("/runs/:runId", async (request, reply) => {
    const runId = (request.params as { runId: string }).runId;
    const record = runs.get(runId);
    if (!record) {
      reply.status(404).send({ error: "Run not found." });
      return;
    }
    reply.send({ runId, status: record.status, manifest: record.manifest, error: record.error });
  });

  app.get("/preview", async (request, reply) => {
    const { assetDir, file } = request.query as { assetDir?: string; file?: string };
    if (!assetDir || !file) {
      reply.status(400).send("Missing assetDir or file query parameters.");
      return;
    }

    const absoluteAssetDir = path.resolve(cwd, assetDir);
    const absoluteFile = path.resolve(absoluteAssetDir, file);
    if (!absoluteFile.startsWith(cwd + path.sep)) {
      reply.status(400).send("Invalid asset path.");
      return;
    }

    if (!fs.existsSync(absoluteFile)) {
      reply.status(404).send("File not found.");
      return;
    }

    const modelUrl = `/file?assetDir=${encodeURIComponent(assetDir)}&file=${encodeURIComponent(
      file,
    )}`;

    const html = `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Meshy Preview</title>
    <script type="module" src="/vendor/model-viewer.min.js"></script>
    <style>
      :root {
        color-scheme: dark;
        --accent-low: #2e1065;
        --accent: #8b5cf6;
        --accent-high: #ddd6fe;
        --bg: #0b0b12;
        --bg-elevated: #12121c;
        --text: #f3f4f6;
      }
      html, body {
        margin: 0;
        padding: 0;
        height: 100%;
        background: var(--bg);
        color: var(--text);
        font-family: "Space Grotesk", "Inter", system-ui, -apple-system, sans-serif;
      }
      .frame {
        display: grid;
        grid-template-rows: auto 1fr;
        height: 100%;
        border-top: 1px solid rgba(139, 92, 246, 0.15);
      }
      header {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 12px 16px;
        background: linear-gradient(90deg, rgba(46, 16, 101, 0.35), rgba(11, 11, 18, 0.95));
        border-bottom: 1px solid rgba(139, 92, 246, 0.2);
        font-size: 13px;
        letter-spacing: 0.04em;
        text-transform: uppercase;
      }
      header span {
        color: var(--accent-high);
      }
      model-viewer {
        width: 100%;
        height: 100%;
        background: radial-gradient(circle at center, #1c1c2b, #0b0b12);
      }
    </style>
  </head>
  <body>
    <div class="frame">
      <header>
        <span>Meshy Preview</span>
        <span>•</span>
        <span>${path.basename(file)}</span>
      </header>
      <model-viewer
        src="${modelUrl}"
        camera-controls
        autoplay
        exposure="1"
        shadow-intensity="0.6"
        ar
      ></model-viewer>
    </div>
  </body>
</html>`;

    reply.type("text/html").send(html);
  });

  app.get("/file", async (request, reply) => {
    const { assetDir, file } = request.query as { assetDir?: string; file?: string };
    if (!assetDir || !file) {
      reply.status(400).send("Missing assetDir or file query parameters.");
      return;
    }

    const absoluteAssetDir = path.resolve(cwd, assetDir);
    const absoluteFile = path.resolve(absoluteAssetDir, file);
    if (!absoluteFile.startsWith(cwd + path.sep)) {
      reply.status(400).send("Invalid asset path.");
      return;
    }

    if (!fs.existsSync(absoluteFile)) {
      reply.status(404).send("File not found.");
      return;
    }

    const ext = path.extname(absoluteFile).toLowerCase();
    const allowedTypes: Record<string, string> = {
      ".glb": "model/gltf-binary",
      ".gltf": "model/gltf+json",
      ".png": "image/png",
      ".jpg": "image/jpeg",
      ".jpeg": "image/jpeg",
    };

    const contentType = allowedTypes[ext];
    if (!contentType) {
      reply.status(403).send("File type not allowed.");
      return;
    }

    reply.type(contentType);
    reply.send(fs.createReadStream(absoluteFile));
  });

  app.get("/vendor/model-viewer.min.js", async (_request, reply) => {
    const vendorPath = path.resolve(
      cwd,
      "node_modules/@google/model-viewer/dist/model-viewer.min.js",
    );
    if (!fs.existsSync(vendorPath)) {
      reply.status(404).send("model-viewer asset not found.");
      return;
    }
    reply.type("text/javascript");
    reply.send(fs.createReadStream(vendorPath));
  });

  return app;
}
