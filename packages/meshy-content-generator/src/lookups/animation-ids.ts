import path from "node:path";
import { fileURLToPath } from "node:url";
import { readJson } from "../utils/json.js";

export interface AnimationLibrary {
  name: string;
  description: string;
  source: string;
  syncedAt: string;
  totalAnimations: number;
  byPath: Record<string, number>;
}

export function loadAnimationIds(): Record<string, number> {
  const __dirname = path.dirname(fileURLToPath(import.meta.url));
  const filePath = path.resolve(__dirname, "../../lookups/animation-library.json");
  const library = readJson<AnimationLibrary>(filePath);
  return library.byPath;
}
