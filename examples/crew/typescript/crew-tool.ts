/**
 * CrewTool - TypeScript client for invoking agentic-crew CLI
 *
 * This is a ready-to-use implementation for integrating agentic-crew
 * into TypeScript/Node.js applications.
 *
 * @example
 * ```typescript
 * import { CrewTool } from './crew-tool';
 *
 * const crewTool = new CrewTool();
 *
 * // List available crews
 * const crews = await crewTool.listCrews();
 *
 * // Invoke a crew
 * const result = await crewTool.invokeCrew({
 *   package: 'vendor-connectors',
 *   crew: 'connector_builder',
 *   input: 'Build a connector for the Meshy API',
 * });
 * ```
 *
 * @see https://github.com/jbcom/agentic-crew/blob/main/docs/INTEGRATION.md
 */

import { spawn, spawnSync } from 'child_process';
import type {
  CrewListResult,
  CrewInfoResult,
  CrewRunResult,
  InvokeCrewParams,
  ExitCode,
} from './agentic-crew-types';

export interface CrewToolOptions {
  /**
   * Path to agentic-crew executable.
   * Default: 'agentic-crew' (uses PATH)
   */
  executable?: string;

  /**
   * Default timeout in milliseconds.
   * Default: 300000 (5 minutes)
   */
  defaultTimeoutMs?: number;

  /**
   * Additional environment variables to pass to the subprocess.
   * API keys can be passed here instead of relying on process.env.
   */
  env?: Record<string, string>;

  /**
   * Enable debug logging.
   * Default: false
   */
  debug?: boolean;
}

export class CrewTool {
  private readonly executable: string;
  private readonly defaultTimeoutMs: number;
  private readonly env: Record<string, string>;
  private readonly debug: boolean;

  constructor(options: CrewToolOptions = {}) {
    this.executable = options.executable ?? 'agentic-crew';
    this.defaultTimeoutMs = options.defaultTimeoutMs ?? 300000;
    this.env = { ...process.env, ...options.env } as Record<string, string>;
    this.debug = options.debug ?? false;
  }

  /**
   * List all available crews.
   *
   * @param packageFilter - Optional package name to filter by
   * @returns List of available crews
   */
  async listCrews(packageFilter?: string): Promise<CrewListResult> {
    const args = ['list'];
    if (packageFilter) {
      args.push(packageFilter);
    }
    args.push('--json');

    const result = await this.execute(args);
    return JSON.parse(result.stdout) as CrewListResult;
  }

  /**
   * List all available crews (synchronous version).
   */
  listCrewsSync(packageFilter?: string): CrewListResult {
    const args = ['list'];
    if (packageFilter) {
      args.push(packageFilter);
    }
    args.push('--json');

    const result = this.executeSync(args);
    return JSON.parse(result.stdout) as CrewListResult;
  }

  /**
   * Get detailed information about a specific crew.
   *
   * @param packageName - Package containing the crew
   * @param crewName - Name of the crew
   * @returns Detailed crew information
   */
  async getCrewInfo(packageName: string, crewName: string): Promise<CrewInfoResult> {
    const args = ['info', packageName, crewName, '--json'];
    const result = await this.execute(args);
    return JSON.parse(result.stdout) as CrewInfoResult;
  }

  /**
   * Invoke a crew and return the result.
   *
   * @param params - Invocation parameters
   * @returns Crew execution result
   */
  async invokeCrew(params: InvokeCrewParams): Promise<CrewRunResult> {
    const args = ['run', params.package, params.crew, '--input', params.input, '--json'];

    if (params.framework && params.framework !== 'auto') {
      args.push('--framework', params.framework);
    }

    const timeoutMs = params.timeoutMs ?? this.defaultTimeoutMs;

    try {
      const result = await this.execute(args, timeoutMs);
      return JSON.parse(result.stdout) as CrewRunResult;
    } catch (error) {
      // Try to parse error output as JSON
      if (error instanceof ExecutionError && error.stdout) {
        try {
          return JSON.parse(error.stdout) as CrewRunResult;
        } catch {
          // Fall through to generic error
        }
      }

      // Return structured error response
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
        duration_ms: 0,
      };
    }
  }

  /**
   * Invoke a crew (synchronous version).
   * Use with caution - this blocks the event loop.
   */
  invokeCrewSync(params: InvokeCrewParams): CrewRunResult {
    const args = ['run', params.package, params.crew, '--input', params.input, '--json'];

    if (params.framework && params.framework !== 'auto') {
      args.push('--framework', params.framework);
    }

    try {
      const result = this.executeSync(args, params.timeoutMs ?? this.defaultTimeoutMs);
      return JSON.parse(result.stdout) as CrewRunResult;
    } catch (error) {
      if (error instanceof ExecutionError && error.stdout) {
        try {
          return JSON.parse(error.stdout) as CrewRunResult;
        } catch {
          // Fall through
        }
      }

      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
        duration_ms: 0,
      };
    }
  }

  /**
   * Check if a specific crew exists.
   */
  async crewExists(packageName: string, crewName: string): Promise<boolean> {
    try {
      const list = await this.listCrews(packageName);
      return list.crews.some((c) => c.package === packageName && c.name === crewName);
    } catch {
      return false;
    }
  }

  /**
   * Get list of available packages.
   */
  async getPackages(): Promise<string[]> {
    const list = await this.listCrews();
    const packages = new Set(list.crews.map((c) => c.package));
    return Array.from(packages);
  }

  // ============================================================================
  // Private execution methods
  // ============================================================================

  private async execute(
    args: string[],
    timeoutMs: number = this.defaultTimeoutMs
  ): Promise<{ stdout: string; stderr: string; exitCode: number }> {
    return new Promise((resolve, reject) => {
      if (this.debug) {
        console.log(`[CrewTool] Executing: ${this.executable} ${args.join(' ')}`);
      }

      const proc = spawn(this.executable, args, {
        env: this.env,
        stdio: ['pipe', 'pipe', 'pipe'],
      });

      let stdout = '';
      let stderr = '';

      proc.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      proc.stderr.on('data', (data) => {
        stderr += data.toString();
        if (this.debug) {
          console.log(`[CrewTool] stderr: ${data.toString().trim()}`);
        }
      });

      const timeoutHandle = setTimeout(() => {
        proc.kill('SIGTERM');
        reject(new Error(`Command timed out after ${timeoutMs}ms`));
      }, timeoutMs);

      proc.on('error', (error) => {
        clearTimeout(timeoutHandle);
        reject(new ExecutionError(`Failed to execute command: ${error.message}`, stdout, stderr, -1));
      });

      proc.on('close', (code) => {
        clearTimeout(timeoutHandle);

        if (this.debug) {
          console.log(`[CrewTool] Exit code: ${code}`);
        }

        const exitCode = code ?? -1;

        if (exitCode === 0) {
          resolve({ stdout, stderr, exitCode });
        } else {
          reject(new ExecutionError(`Command failed with exit code ${exitCode}`, stdout, stderr, exitCode));
        }
      });
    });
  }

  private executeSync(
    args: string[],
    timeoutMs: number = this.defaultTimeoutMs
  ): { stdout: string; stderr: string; exitCode: number } {
    if (this.debug) {
      console.log(`[CrewTool] Executing (sync): ${this.executable} ${args.join(' ')}`);
    }

    const result = spawnSync(this.executable, args, {
      env: this.env,
      timeout: timeoutMs,
      encoding: 'utf-8',
    });

    if (result.error) {
      throw new ExecutionError(
        `Failed to execute command: ${result.error.message}`,
        result.stdout ?? '',
        result.stderr ?? '',
        result.status ?? -1
      );
    }

    const exitCode = result.status ?? -1;

    if (exitCode !== 0) {
      throw new ExecutionError(
        `Command failed with exit code ${exitCode}`,
        result.stdout ?? '',
        result.stderr ?? '',
        exitCode
      );
    }

    return {
      stdout: result.stdout ?? '',
      stderr: result.stderr ?? '',
      exitCode,
    };
  }
}

/**
 * Custom error class for execution failures
 */
export class ExecutionError extends Error {
  constructor(
    message: string,
    public readonly stdout: string,
    public readonly stderr: string,
    public readonly exitCode: number
  ) {
    super(message);
    this.name = 'ExecutionError';
  }

  /**
   * Get the exit code as an ExitCode enum value
   */
  get exitCodeEnum(): ExitCode | undefined {
    if (this.exitCode >= 0 && this.exitCode <= 3) {
      return this.exitCode as ExitCode;
    }
    return undefined;
  }
}
