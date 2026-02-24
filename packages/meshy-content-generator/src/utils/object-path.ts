export function getPathValue(obj: unknown, path: string): unknown {
  if (!path) return undefined;
  const parts = path.split(".").filter(Boolean);
  let current: unknown = obj;
  for (const part of parts) {
    if (current === null || current === undefined) return undefined;
    if (typeof current !== "object") return undefined;
    const record = current as Record<string, unknown>;
    current = record[part];
  }
  return current;
}

export function setPathValue(obj: Record<string, unknown>, path: string, value: unknown): void {
  const parts = path.split(".").filter(Boolean);
  if (parts.length === 0) return;
  let current: Record<string, unknown> = obj;
  for (let i = 0; i < parts.length - 1; i += 1) {
    const part = parts[i] as string;
    const next = current[part];
    if (!next || typeof next !== "object") {
      current[part] = {};
    }
    current = current[part] as Record<string, unknown>;
  }
  const lastPart = parts[parts.length - 1];
  if (lastPart !== undefined) {
    current[lastPart] = value;
  }
}
