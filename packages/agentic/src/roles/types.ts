import type { AgentConfig } from '../triage/agent.js';

export const ROLE_NAMES = ['harvester', 'curator', 'reviewer', 'fixer', 'delegator'] as const;
export type RoleName = (typeof ROLE_NAMES)[number];
export type AgentCapability = 'review' | 'triage' | 'fix' | 'merge' | 'delegate';
export type RoleTrigger =
  | { type: 'command'; value: string }
  | { type: 'event'; value: string }
  | { type: 'schedule'; value: string };

export interface RoleDefinition {
  name: RoleName;
  description: string;
  systemPrompt: string;
  capabilities: readonly AgentCapability[];
  triggers: readonly RoleTrigger[];
}

export interface RoleConfig {
  enabled?: boolean;
  model?: string;
  systemPrompt?: string;
  maxSteps?: number;
}

export type RolesConfig = Partial<Record<RoleName, RoleConfig>>;
export interface EffectiveRole extends RoleDefinition {
  enabled: boolean;
  model?: string;
  maxSteps?: number;
}

export interface RoleExecutionOptions {
  /** Overrides the configured roles for this call. */
  roles?: RolesConfig;
  /** Existing tool approval, MCP, and sandbox settings. */
  agent?: AgentConfig;
}
