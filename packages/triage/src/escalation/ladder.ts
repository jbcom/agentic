/**
 * Escalation Ladder
 *
 * Implements a 7-level escalation strategy that exhausts all free options
 * before resorting to expensive cloud agents.
 *
 * Levels:
 * - 0: Static Analysis (lint/tsc) - Free, instant
 * - 1: Complexity Evaluation (Ollama) - Free, routes to 2 or 3
 * - 2: Ollama Fix - Free, simple fixes
 * - 3: Jules Session - Free tier, complex work
 * - 4: Jules + Boosted Context - Free tier, more context
 * - 5: Human Review Queue - Free, awaits approval
 * - 6: Cloud Agent (Cursor) - Expensive, requires approval
 */

import type { EscalationConfig } from './config.js';
import { createEscalationConfig } from './config.js';
import { CostTracker } from './cost-tracker.js';
import { type EscalationLevel, type EscalationState, EscalationStateManager } from './state.js';

/**
 * Task to be processed by the escalation ladder
 */
export interface Task {
    /** Unique task identifier */
    id: string;
    /** Task description */
    description: string;
    /** Code/context for the task */
    context: string;
    /** Task metadata (e.g., labels, approvals) */
    metadata?: Record<string, unknown>;
}

/**
 * Result from processing a task
 */
export interface ProcessResult {
    /** Whether the task was resolved */
    success: boolean;
    /** Final escalation level reached */
    level: EscalationLevel;
    /** Result data (agent-specific) */
    data?: unknown;
    /** Error message if failed */
    error?: string;
    /** Total cost incurred (in cents) */
    cost: number;
    /** Number of attempts made */
    attempts: number;
    /** Trail of levels attempted */
    trail: Array<{ level: EscalationLevel; success: boolean; error?: string }>;
}

/**
 * Handler function for a specific escalation level
 */
export type LevelHandler = (
    task: Task,
    state: EscalationState
) => Promise<{
    success: boolean;
    data?: unknown;
    error?: string;
    escalate: boolean;
    cost?: number;
}>;

/**
 * Escalation Ladder - intelligently routes tasks through 7 levels
 */
export class EscalationLadder {
    private config: EscalationConfig;
    private stateManager: EscalationStateManager;
    private costTracker: CostTracker;
    private handlers: Map<EscalationLevel, LevelHandler> = new Map();

    constructor(config?: Partial<EscalationConfig>) {
        this.config = createEscalationConfig(config);
        this.stateManager = new EscalationStateManager();
        this.costTracker = new CostTracker(this.config.costBudgetDaily);
    }

    /**
     * Register a handler for a specific level
     */
    registerHandler(level: EscalationLevel, handler: LevelHandler): this {
        this.handlers.set(level, handler);
        return this;
    }

    /**
     * Process a task through the escalation ladder
     */
    async process(task: Task): Promise<ProcessResult> {
        const trail: ProcessResult['trail'] = [];
        let totalCost = 0;
        let currentLevel = this.initializeTask(task);

        while (currentLevel <= 6) {
            let state = this.stateManager.getState(task.id);
            const execution = this.prepareLevelExecution(task.id, currentLevel, state, trail);
            currentLevel = execution.level;
            if (!execution.handler) {
                continue;
            }

            this.stateManager.recordAttempt(task.id, currentLevel);
            state = this.stateManager.getState(task.id);
            const outcome = await this.executeLevel(task, currentLevel, state, execution.handler, trail, totalCost);
            totalCost = outcome.totalCost;

            if (outcome.result) {
                return outcome.result;
            }

            currentLevel = outcome.nextLevel;
        }

        return this.buildFailureResult(trail, totalCost);
    }

    private initializeTask(task: Task): EscalationLevel {
        if (this.hasCloudAgentApproval(task)) {
            this.stateManager.setApproval(task.id, true);
        }

        return this.stateManager.getState(task.id).level;
    }

    private prepareLevelExecution(
        taskId: string,
        currentLevel: EscalationLevel,
        state: EscalationState,
        trail: ProcessResult['trail']
    ): { level: EscalationLevel; handler: LevelHandler | null } {
        if (this.shouldSkipLevel(currentLevel, state)) {
            return { level: this.advanceLevel(taskId, currentLevel), handler: null };
        }

        const handler = this.handlers.get(currentLevel);
        if (!handler) {
            trail.push({ level: currentLevel, success: false, error: 'No handler registered' });
            return { level: this.advanceLevel(taskId, currentLevel), handler: null };
        }

        if (this.hasExceededAttempts(currentLevel, state)) {
            trail.push({ level: currentLevel, success: false, error: 'Max attempts exceeded' });
            return { level: this.advanceLevel(taskId, currentLevel), handler: null };
        }

        return { level: currentLevel, handler };
    }

    private advanceLevel(taskId: string, currentLevel: EscalationLevel): EscalationLevel {
        const nextLevel = (currentLevel + 1) as EscalationLevel;
        this.stateManager.updateState(taskId, { level: nextLevel });
        return nextLevel;
    }

    private recordExecutionCost(taskId: string, level: EscalationLevel, cost?: number): number {
        if (!cost) {
            return 0;
        }

        this.stateManager.addCost(taskId, cost);
        if (level === 6) {
            this.costTracker.record(taskId, 'cloud-agent', cost, `Level ${level} execution`);
        }

        return cost;
    }

    private async executeLevel(
        task: Task,
        currentLevel: EscalationLevel,
        state: EscalationState,
        handler: LevelHandler,
        trail: ProcessResult['trail'],
        totalCost: number
    ): Promise<{ nextLevel: EscalationLevel; totalCost: number; result?: ProcessResult }> {
        try {
            const result = await handler(task, state);
            totalCost += this.recordExecutionCost(task.id, currentLevel, result.cost);
            trail.push({ level: currentLevel, success: result.success, error: result.error });

            if (result.success) {
                this.stateManager.resolve(task.id);
                return {
                    nextLevel: currentLevel,
                    totalCost,
                    result: {
                        success: true,
                        level: currentLevel,
                        data: result.data,
                        cost: totalCost,
                        attempts: trail.length,
                        trail,
                    },
                };
            }

            if (result.error) {
                this.stateManager.recordError(task.id, result.error);
            }

            const freshState = this.stateManager.getState(task.id);
            const nextLevel =
                result.escalate || this.hasExceededAttempts(currentLevel, freshState)
                    ? this.advanceLevel(task.id, currentLevel)
                    : currentLevel;

            return { nextLevel, totalCost };
        } catch (error) {
            const errorMsg = error instanceof Error ? error.message : 'Unknown error';
            this.stateManager.recordError(task.id, errorMsg);
            trail.push({ level: currentLevel, success: false, error: errorMsg });

            return {
                nextLevel: this.advanceLevel(task.id, currentLevel),
                totalCost,
            };
        }
    }

    private buildFailureResult(trail: ProcessResult['trail'], totalCost: number): ProcessResult {
        return {
            success: false,
            level: 6,
            error: 'All escalation levels exhausted',
            cost: totalCost,
            attempts: trail.length,
            trail,
        };
    }

    /**
     * Get state for a task
     */
    getState(taskId: string): EscalationState {
        return this.stateManager.getState(taskId);
    }

    /**
     * Reset state for a task
     */
    resetState(taskId: string): void {
        this.stateManager.resetState(taskId);
    }

    /**
     * Get all states
     */
    getAllStates(): EscalationState[] {
        return this.stateManager.getAllStates();
    }

    /**
     * Get cost tracker
     */
    getCostTracker(): CostTracker {
        return this.costTracker;
    }

    /**
     * Get configuration
     */
    getConfig(): EscalationConfig {
        return { ...this.config };
    }

    /**
     * Update configuration
     */
    updateConfig(update: Partial<EscalationConfig>): void {
        this.config = { ...this.config, ...update };
        this.costTracker.setDailyBudget(this.config.costBudgetDaily);
    }

    /**
     * Check if level should be skipped
     */
    private shouldSkipLevel(level: EscalationLevel, state: EscalationState): boolean {
        // Level 5 (Human Review) - skip if already approved or approval not required
        if (level === 5) {
            return state.approved || !this.config.cloudAgentApprovalRequired;
        }

        // Level 6 (Cloud Agent) - check if enabled and budget available
        if (level === 6) {
            if (!this.config.cloudAgentEnabled) return true;
            if (this.config.cloudAgentApprovalRequired && !state.approved) return true;

            // Check budget (estimate cloud agent cost at 1000 cents = $10)
            const estimatedCost = 1000;
            if (!this.costTracker.canAfford(estimatedCost)) return true;
        }

        return false;
    }

    /**
     * Check if level has exceeded max attempts
     */
    private hasExceededAttempts(level: EscalationLevel, state: EscalationState): boolean {
        const levelKey = `level${level}`;
        const attempts = state.attempts[levelKey] || 0;

        switch (level) {
            case 2: // Ollama Fix
                return attempts >= this.config.maxOllamaAttempts;
            case 3: // Jules Session
                return attempts >= this.config.maxJulesAttempts;
            case 4: // Jules + Boosted Context
                return attempts >= this.config.maxJulesBoostAttempts;
            default:
                // Other levels: 1 attempt only
                return attempts >= 1;
        }
    }

    /**
     * Check if task has cloud agent approval
     */
    private hasCloudAgentApproval(task: Task): boolean {
        if (!task.metadata) return false;

        // Check for 'approved:cloud-agent' in labels
        const labels = task.metadata.labels as string[] | undefined;
        if (labels?.includes('approved:cloud-agent')) return true;

        // Check for explicit approval flag
        const approved = task.metadata.approved as boolean | string[] | undefined;
        if (typeof approved === 'boolean') return approved;
        if (Array.isArray(approved)) return approved.includes('cloud-agent');

        return false;
    }
}
