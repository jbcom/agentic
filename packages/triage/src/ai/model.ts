import { anthropic } from '@ai-sdk/anthropic';
import { google } from '@ai-sdk/google';
import { createOllama, ollama } from 'ai-sdk-ollama';
import type { AIConfig } from './types.js';

// Default model - qwen3-coder:480b on Ollama Cloud has excellent tool support
export const DEFAULT_MODEL = 'qwen3-coder:480b';
export const CLOUD_HOST = 'https://ollama.com/api';
export const LOCAL_HOST = 'http://localhost:11434/api';

/**
 * Get or create a Ollama provider instance
 * Uses the default `ollama` export for local, or createOllama for cloud
 */
export function getProvider(config: AIConfig = {}) {
    const apiKey = config.apiKey || process.env.OLLAMA_API_KEY;
    const hostEnv = config.host || process.env.OLLAMA_HOST;

    // If no custom config needed, use default provider
    if (!apiKey && !hostEnv) {
        return ollama;
    }

    // Normalize host URL - ensure it ends with /api for Ollama endpoints
    let host = hostEnv || (apiKey ? CLOUD_HOST : LOCAL_HOST);
    if (host && !host.endsWith('/api')) {
        host = `${host.replace(/\/$/, '')}/api`;
    }

    // Create custom provider with auth headers for cloud
    // Note: We don't cache this globally to avoid issues with different configurations
    return createOllama({
        baseURL: host,
        headers: apiKey ? { Authorization: `Bearer ${apiKey}` } : undefined,
    });
}

/**
 * Get the model ID to use
 */
export function getModel(config: AIConfig = {}): string {
    return config.model || process.env.OLLAMA_MODEL || DEFAULT_MODEL;
}

/**
 * Resolve the Vercel AI SDK model instance to use.
 */
export async function resolveModel(config: AIConfig = {}): Promise<{
    providerName: string;
    modelId: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    model: any;
}> {
    const providerName = config.provider || process.env.TRIAGE_PROVIDER || 'ollama';

    if (providerName === 'ollama') {
        const provider = getProvider(config);
        const modelId = getModel(config);
        return { providerName, modelId, model: provider(modelId) };
    }

    if (providerName === 'anthropic') {
        const modelId = config.model || process.env.ANTHROPIC_MODEL || 'claude-3-5-sonnet-20241022';
        return { providerName, modelId, model: anthropic(modelId) };
    }

    if (providerName === 'google' || providerName === 'gemini') {
        const modelId = config.model || process.env.GOOGLE_MODEL || 'gemini-1.5-pro';
        return { providerName, modelId, model: google(modelId) };
    }

    throw new Error(
        `Provider ${providerName} is not supported. Supported providers: ollama, anthropic, google.` +
            `For other providers, please use the direct AI SDK integration or configure via environment variables.`
    );
}
