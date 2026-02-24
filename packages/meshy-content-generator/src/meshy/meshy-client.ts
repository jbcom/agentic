/**
 * Meshy API Client
 *
 * Base client with authentication, error handling, and rate limiting.
 */

export interface MeshyClientConfig {
  /** Meshy API key. */
  apiKey: string;
  /** Override base URL for Meshy (default: https://api.meshy.ai/openapi). */
  baseUrl?: string;
  /** SSE stream timeout in milliseconds. */
  streamTimeoutMs?: number;
  /** Max retries on rate-limit errors. */
  maxRetries?: number;
  /** Base delay (ms) between retries. */
  retryDelayMs?: number;
}

export type TaskStatus = "PENDING" | "IN_PROGRESS" | "SUCCEEDED" | "FAILED" | "CANCELED";

export interface TaskResult<T = unknown> {
  id: string;
  status: TaskStatus;
  progress: number;
  result?: T;
  task_error?: { message: string };
}

export class MeshyApiError extends Error {
  constructor(
    message: string,
    public readonly statusCode: number,
    public readonly isRateLimit = false,
    public readonly isAuthError = false,
  ) {
    super(message);
    this.name = "MeshyApiError";
  }
}

export class RateLimitError extends MeshyApiError {
  constructor(message: string) {
    super(message, 429, true, false);
    this.name = "RateLimitError";
  }
}

export class AuthenticationError extends MeshyApiError {
  constructor(message: string) {
    super(message, 401, false, true);
    this.name = "AuthenticationError";
  }
}

export class InsufficientCreditsError extends MeshyApiError {
  constructor(message: string) {
    super(message, 402, false, false);
    this.name = "InsufficientCreditsError";
  }
}

/**
 * Low-level Meshy API client with streaming support.
 */
export class MeshyClient {
  private readonly apiKey: string;
  private readonly baseUrl: string;
  private readonly streamTimeoutMs: number;
  private readonly maxRetries: number;
  private readonly retryDelayMs: number;

  constructor(config: MeshyClientConfig) {
    this.apiKey = config.apiKey;
    this.baseUrl = (config.baseUrl ?? "https://api.meshy.ai/openapi").replace(/\/$/, "");
    this.streamTimeoutMs = config.streamTimeoutMs ?? 600_000;
    this.maxRetries = config.maxRetries ?? 3;
    this.retryDelayMs = config.retryDelayMs ?? 1000;
  }

  /**
   * Perform a GET request against the Meshy API.
   *
   * @param path - API path (e.g. /v1/text-to-image).
   * @param query - Optional query params.
   */
  async get<T = unknown>(
    path: string,
    query?: Record<string, string | number | boolean>,
  ): Promise<T> {
    return this.request<T>("GET", path, undefined, query);
  }

  /**
   * Perform a POST request against the Meshy API.
   *
   * @param path - API path (e.g. /v2/text-to-3d).
   * @param body - JSON payload for Meshy.
   */
  async post<T = unknown>(path: string, body: unknown): Promise<T> {
    return this.request<T>("POST", path, body);
  }

  /**
   * Perform a DELETE request against the Meshy API.
   *
   * @param path - API path to delete.
   */
  async delete(path: string): Promise<void> {
    await this.request("DELETE", path);
  }

  /**
   * Stream task progress (SSE) until completion.
   *
   * @param path - Stream endpoint for a Meshy task.
   */
  async streamUntilComplete<T = unknown>(path: string): Promise<TaskResult<T>> {
    const url = this.buildUrl(path);

    const response = await fetch(url, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${this.apiKey}`,
        Accept: "text/event-stream",
      },
      signal: AbortSignal.timeout(this.streamTimeoutMs),
    });

    await this.ensureOk(response);

    if (!response.body) {
      throw new MeshyApiError("Streaming response did not include a body", 500);
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    let buffer = "";
    let finalResult: TaskResult<T> | null = null;

    try {
      while (true) {
        const { value, done } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });
        const lines = buffer.split(/\r?\n/);
        buffer = lines.pop() ?? "";

        for (const line of lines) {
          if (!line.trim().startsWith("data:")) continue;

          const payload = line.replace(/^data:\s*/, "");
          try {
            const parsed = JSON.parse(payload) as TaskResult<T>;
            finalResult = parsed;

            if (["SUCCEEDED", "FAILED", "CANCELED"].includes(parsed.status)) {
              await reader.cancel();
              return parsed;
            }
          } catch {
            // Ignore parse errors for non-JSON lines
          }
        }
      }
    } finally {
      reader.releaseLock();
    }

    if (finalResult) return finalResult;
    throw new MeshyApiError("No data received from stream", 500);
  }

  /**
   * Core HTTP handler with retry/backoff for rate limits.
   */
  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
    query?: Record<string, string | number | boolean>,
  ): Promise<T> {
    let lastError: Error | null = null;

    for (let attempt = 1; attempt <= this.maxRetries + 1; attempt += 1) {
      try {
        const url = this.buildUrl(path, query);
        const response = await fetch(url, {
          method,
          headers: {
            Authorization: `Bearer ${this.apiKey}`,
            "Content-Type": "application/json",
          },
          body: body ? JSON.stringify(body) : null,
        });

        await this.ensureOk(response);

        if (response.status === 204) {
          return undefined as T;
        }

        return (await response.json()) as T;
      } catch (error) {
        lastError = error as Error;
        if (error instanceof RateLimitError && attempt <= this.maxRetries) {
          await this.delay(this.retryDelayMs * attempt);
          continue;
        }
        break;
      }
    }

    throw lastError ?? new MeshyApiError("Unknown Meshy error", 500);
  }

  /**
   * Build a full API URL with query parameters.
   */
  private buildUrl(path: string, query?: Record<string, string | number | boolean>): string {
    const url = new URL(`${this.baseUrl}${path.startsWith("/") ? "" : "/"}${path}`);
    if (query) {
      for (const [key, value] of Object.entries(query)) {
        url.searchParams.set(key, String(value));
      }
    }
    return url.toString();
  }

  /**
   * Validate HTTP responses and surface Meshy errors.
   */
  private async ensureOk(response: Response): Promise<void> {
    if (response.ok) return;

    let message = response.statusText;
    try {
      const payload = (await response.json()) as { message?: string };
      if (payload.message) message = payload.message;
    } catch {
      // Ignore JSON parse errors
    }

    if (response.status === 401) throw new AuthenticationError(message);
    if (response.status === 402) throw new InsufficientCreditsError(message);
    if (response.status === 429) throw new RateLimitError(message);

    throw new MeshyApiError(message, response.status);
  }

  /**
   * Delay helper for retry backoff.
   */
  private async delay(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}
