import { AgentCapabilities, AgentDefinition } from '@agentic/triage';

/**
 * Google Jules Provider Implementation
 *
 * Creates agents that use Google Jules for complex async tasks.
 * Jules can create PRs, run commands, and handle multi-file changes.
 */

interface JulesConfig {
    /** Jules API key */
    apiKey: string;
    /** API base URL (default: https://jules.googleapis.com/v1alpha) */
    baseUrl?: string;
    /** Automation mode (default: AUTO_CREATE_PR) */
    automationMode?: 'AUTOMATION_MODE_UNSPECIFIED' | 'AUTO_CREATE_PR' | 'MANUAL';
}
interface JulesSessionResult {
    /** Session ID for polling */
    sessionId: string;
    /** Session name/resource path */
    name: string;
    /** Current state */
    state: string;
}
/**
 * Create a Jules-based agent for the registry
 */
declare function createJulesAgent(id: string, config: JulesConfig, options?: {
    name?: string;
    cost?: number;
    priority?: number;
    capabilities?: Partial<AgentCapabilities>;
}): AgentDefinition<JulesSessionResult>;
/**
 * Poll a Jules session for completion
 */
declare function pollJulesSession(config: JulesConfig, sessionName: string): Promise<{
    state: string;
    prUrl?: string;
    error?: string;
}>;
/**
 * Send a follow-up message to a Jules session
 */
declare function sendJulesFollowUp(config: JulesConfig, sessionName: string, message: string): Promise<void>;

export { type JulesConfig, type JulesSessionResult, createJulesAgent, pollJulesSession, sendJulesFollowUp };
