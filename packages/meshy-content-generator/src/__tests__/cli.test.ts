import { spawnSync } from "node:child_process";
import path from "node:path";
import { describe, expect, it } from "vitest";

const root = path.resolve(process.cwd());

function runCli(args: string[]) {
  const result = spawnSync("pnpm", ["exec", "tsx", "src/cli/index.ts", ...args], {
    cwd: root,
    encoding: "utf-8",
    env: {
      ...process.env,
      DOTENV_CONFIG_QUIET: "true",
    },
  });

  return {
    stdout: result.stdout?.trim() ?? "",
    stderr: result.stderr?.trim() ?? "",
    status: result.status ?? 0,
  };
}

describe("CLI", () => {
  it("lists pipelines and tasks", () => {
    const { stdout, status } = runCli([
      "list",
      "--pipelines",
      "pipelines/definitions",
      "--tasks",
      "tasks/definitions",
    ]);

    expect(status).toBe(0);
    expect(stdout).toMatchInlineSnapshot(`
      "Pipelines:\n- character\n- character-full\n- prop\n\nTasks:\n- animation\n- multi-image-to-3d\n- rigging\n- text-to-3d-preview\n- text-to-3d-refine\n- text-to-image"
    `);
  });
});
