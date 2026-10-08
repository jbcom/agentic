import { getConfig, getDefaultModel } from '../core/config.js';
import { validateConfig } from '../core/validation.js';
import type { AgentResult } from '../triage/agent.js';
import { DEFAULT_ROLES } from './defaults.js';
import type {
  EffectiveRole,
  RoleExecutionOptions,
  RoleName,
  RolesConfig,
  RoleTrigger,
} from './types.js';
import { ROLE_NAMES } from './types.js';

export function getEffectiveRole(name: string, roles: RolesConfig = {}): EffectiveRole | undefined {
  if (!ROLE_NAMES.includes(name as RoleName)) return undefined;
  validateConfig({ roles });
  const definition = DEFAULT_ROLES[name as RoleName];
  const configured = getConfig().roles?.[name as RoleName];
  const override = { ...configured, ...roles[name as RoleName] };
  return {
    ...definition,
    capabilities: [...definition.capabilities],
    triggers: definition.triggers.map((trigger) => ({ ...trigger })),
    enabled: override?.enabled ?? true,
    model: override?.model,
    maxSteps: override?.maxSteps,
    systemPrompt: override?.systemPrompt ?? definition.systemPrompt,
  };
}

export function listRoles(roles?: RolesConfig): EffectiveRole[] {
  return ROLE_NAMES.map((name) => getEffectiveRole(name, roles) as EffectiveRole);
}

/** String input matches exact event/schedule values or the first command token. */
export function findRoleByTrigger(
  trigger: RoleTrigger | string,
  roles?: RolesConfig
): EffectiveRole | undefined {
  return listRoles(roles).find(
    (role) =>
      role.enabled &&
      role.triggers.some((candidate) => {
        if (typeof trigger !== 'string')
          return candidate.type === trigger.type && candidate.value === trigger.value;
        const value =
          candidate.type === 'command' ? trigger.trim().split(/\s+/)[0] : trigger.trim();
        return candidate.value === value;
      })
  );
}

export async function executeRole(
  name: string,
  task: string,
  options: RoleExecutionOptions = {}
): Promise<AgentResult> {
  const role = getEffectiveRole(name, options.roles);
  if (!role) throw new RangeError(`Unknown agent role: ${name}`);
  if (!role.enabled) throw new Error(`Agent role is disabled: ${name}`);
  if (!task.trim()) throw new TypeError('Role task must not be empty');
  if (
    options.agent?.approval?.requireApproval?.length &&
    !options.agent.approval.onApprovalRequest
  ) {
    throw new TypeError('Role approval requirements need an onApprovalRequest callback');
  }
  const triage = getConfig().triage;
  const defaultModel =
    !triage?.provider || triage.provider === 'anthropic' ? getDefaultModel() : undefined;
  const { Agent } = await import('../triage/agent.js');
  const agent = new Agent({
    ...options.agent,
    model: role.model ?? options.agent?.model ?? defaultModel,
    maxSteps: role.maxSteps ?? options.agent?.maxSteps,
    systemPrompt: [role.systemPrompt, options.agent?.systemPrompt].filter(Boolean).join('\n\n'),
  });
  try {
    return await agent.execute(task);
  } finally {
    await agent.close();
  }
}
