const TEMPLATE_REGEX = /\{\{\s*([\w.-]+)\s*\}\}/g;

export function applyTemplate(value: string, vars: Record<string, unknown>): string {
  return value.replace(TEMPLATE_REGEX, (_match, key) => {
    const resolved = vars[key];
    return resolved === undefined || resolved === null ? "" : String(resolved);
  });
}
