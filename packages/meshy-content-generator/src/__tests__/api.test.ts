import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { buildServer } from "../api/app.js";

const baseDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../");

describe("API server", () => {
  it("serves OpenAPI spec", async () => {
    const app = buildServer({ cwd: baseDir });
    const response = await app.inject({ method: "GET", url: "/openapi.json" });
    expect(response.statusCode).toBe(200);
    const payload = JSON.parse(response.payload);
    expect(payload.info.title).toBe("Meshy Content Generator API");
    await app.close();
  });

  it("serves preview HTML", async () => {
    const tempDir = path.join(baseDir, ".tmp-preview");
    fs.mkdirSync(tempDir, { recursive: true });
    const glbPath = path.join(tempDir, "model.glb");
    fs.writeFileSync(glbPath, "glTF");

    const app = buildServer({ cwd: baseDir });
    const response = await app.inject({
      method: "GET",
      url: `/preview?assetDir=${encodeURIComponent(".tmp-preview")}&file=model.glb`,
    });
    expect(response.statusCode).toBe(200);
    expect(response.payload).toContain("model-viewer");
    await app.close();
  });

  it("serves asset files via /file", async () => {
    const tempDir = path.join(baseDir, ".tmp-preview");
    fs.mkdirSync(tempDir, { recursive: true });
    const glbPath = path.join(tempDir, "model.glb");
    fs.writeFileSync(glbPath, "glTF");

    const app = buildServer({ cwd: baseDir });
    const response = await app.inject({
      method: "GET",
      url: `/file?assetDir=${encodeURIComponent(".tmp-preview")}&file=model.glb`,
    });
    expect(response.statusCode).toBe(200);
    expect(response.headers["content-type"]).toContain("model/gltf-binary");
    await app.close();
  });

  it("blocks non-preview file types via /file", async () => {
    const tempDir = path.join(baseDir, ".tmp-preview");
    fs.mkdirSync(tempDir, { recursive: true });
    const txtPath = path.join(tempDir, "notes.txt");
    fs.writeFileSync(txtPath, "nope");

    const app = buildServer({ cwd: baseDir });
    const response = await app.inject({
      method: "GET",
      url: `/file?assetDir=${encodeURIComponent(".tmp-preview")}&file=notes.txt`,
    });
    expect(response.statusCode).toBe(403);
    await app.close();
  });
});
