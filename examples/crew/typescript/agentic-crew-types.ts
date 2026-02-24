/**
 * TypeScript types for agentic-crew CLI integration.
 *
 * These types match the JSON output from the agentic-crew CLI when using --json flag.
 * Copy this file to your TypeScript project for type-safe integration.
 *
 * @version 0.1.0
 * @see https://github.com/jbcom/agentic-crew/blob/main/docs/INTEGRATION.md
 */

// ============================================================================
// LIST COMMAND TYPES
// ============================================================================

/**
 * Result from `agentic-crew list --json`
 *
 * @example
 * ```bash
 * agentic-crew list --json
 * ```
 */
export interface CrewListResult {
  crews: CrewSummary[];
}

/**
 * Summary information about a crew
 */
export interface CrewSummary {
  /** Package containing the crew (e.g., "otterfall", "vendor-connectors") */
  package: string;

  /** Crew name (e.g., "game_builder", "connector_builder") */
  name: string;

  /** Human-readable description of what the crew does */
  description: string;

  /**
   * Required framework for this crew.
   * - `null`: Any available framework works (auto-detect)
   * - `"crewai"`: Requires CrewAI framework
   * - `"langgraph"`: Requires LangGraph framework
   * - `"strands"`: Requires AWS Strands framework
   */
  required_framework: Framework | null;
}

// ============================================================================
// INFO COMMAND TYPES
// ============================================================================

/**
 * Result from `agentic-crew info <package> <crew> --json`
 *
 * @example
 * ```bash
 * agentic-crew info vendor-connectors connector_builder --json
 * ```
 */
export interface CrewInfoResult {
  /** Package name */
  package: string;

  /** Crew name */
  name: string;

  /** Human-readable description */
  description: string;

  /** Required framework (null = any framework works) */
  required_framework: Framework | null;

  /** List of agents in this crew */
  agents: AgentInfo[];

  /** List of tasks in this crew */
  tasks: TaskInfo[];

  /** Paths to knowledge directories used by this crew */
  knowledge_paths: string[];
}

/**
 * Information about an agent in a crew
 */
export interface AgentInfo {
  /** Agent identifier (e.g., "api_analyst", "senior_engineer") */
  name: string;

  /** Agent's role description */
  role: string;
}

/**
 * Information about a task in a crew
 */
export interface TaskInfo {
  /** Task identifier (e.g., "analyze_api", "generate_code") */
  name: string;

  /** Task description */
  description: string;
}

// ============================================================================
// RUN COMMAND TYPES
// ============================================================================

/**
 * Result from `agentic-crew run <package> <crew> --input "..." --json`
 *
 * @example
 * ```bash
 * agentic-crew run otterfall game_builder --input "Create a QuestComponent" --json
 * ```
 */
export interface CrewRunResult {
  /** Whether the crew execution was successful */
  success: boolean;

  /**
   * Crew output (present when success=true)
   * Contains the final result/output from the crew's work
   */
  output?: string;

  /**
   * Error message (present when success=false)
   * Describes what went wrong
   */
  error?: string;

  /**
   * Framework that was used for execution.
   * May be absent if the crew failed before framework selection.
   */
  framework_used?: Framework;

  /** Execution duration in milliseconds */
  duration_ms: number;

  /**
   * Available packages (present on "package not found" error)
   * Use this to suggest valid packages to the user
   */
  available_packages?: string[];
}

// ============================================================================
// COMMON TYPES
// ============================================================================

/**
 * Supported AI frameworks
 */
export type Framework = 'crewai' | 'langgraph' | 'strands';

/**
 * Framework selection options for run command
 */
export type FrameworkOption = 'auto' | Framework;

/**
 * CLI exit codes
 *
 * @example
 * ```typescript
 * const result = spawnSync('agentic-crew', args);
 * if (result.status === ExitCode.ConfigurationError) {
 *   console.error('Package or crew not found');
 * }
 * ```
 */
export enum ExitCode {
  /** Crew executed successfully */
  Success = 0,

  /** Crew execution failed (runtime error during execution) */
  ExecutionFailed = 1,

  /** Configuration error (package/crew not found, invalid YAML, etc.) */
  ConfigurationError = 2,

  /** Required framework is not installed */
  FrameworkNotAvailable = 3,
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/**
 * Error response when crew info fails
 */
export interface CrewInfoErrorResult {
  /** Error message */
  error: string;

  /** Available packages (if package not found) */
  available_packages?: string[];
}

/**
 * Type guard to check if CrewRunResult indicates success
 */
export function isSuccessResult(
  result: CrewRunResult
): result is CrewRunResult & { success: true; output: string } {
  return result.success === true && typeof result.output === 'string';
}

/**
 * Type guard to check if CrewRunResult indicates failure
 */
export function isFailureResult(
  result: CrewRunResult
): result is CrewRunResult & { success: false; error: string } {
  return result.success === false;
}

// ============================================================================
// INVOCATION PARAMETER TYPES
// ============================================================================

/**
 * Parameters for invoking a crew
 */
export interface InvokeCrewParams {
  /** Package containing the crew */
  package: string;

  /** Crew name within the package */
  crew: string;

  /** Input/task specification for the crew */
  input: string;

  /**
   * Framework to use.
   * - `"auto"` (default): Auto-detect best available framework
   * - Specific framework: Force that framework
   *
   * Note: If the crew is defined in a framework-specific directory
   * (e.g., `.crewai/`), that framework takes precedence.
   */
  framework?: FrameworkOption;

  /**
   * Timeout in milliseconds.
   * Crews can take significant time to execute.
   * Default: 300000 (5 minutes)
   */
  timeoutMs?: number;
}
