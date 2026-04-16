export interface AIConfig {
    provider?: string;
    apiKey?: string;
    model?: string;
    host?: string;
}

// Use a looser type for tools to avoid version incompatibilities between
// @ai-sdk/mcp and the ai package
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type ToolSet = Record<string, any>;

export interface GenerateOptions extends AIConfig {
    systemPrompt?: string;
    maxTokens?: number;
    temperature?: number;
}

export interface GenerateWithToolsOptions extends GenerateOptions {
    maxSteps?: number;
    onStepFinish?: (step: { toolCalls?: unknown[]; toolResults?: unknown[]; text?: string }) => void;
}

export interface GenerateWithToolsResult {
    text: string;
    toolCalls: unknown[];
    toolResults: unknown[];
    steps: unknown[];
    finishReason: string;
}
