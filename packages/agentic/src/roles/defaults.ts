import type { RoleDefinition, RoleName } from './types.js';

/** Metadata only: schedules and capabilities do not install jobs or grant permissions. */
export const DEFAULT_ROLES: Readonly<Record<RoleName, RoleDefinition>> = {
  harvester: {
    name: 'harvester',
    description: 'Manage PR lifecycle, reviews, and verified merges.',
    systemPrompt:
      'Act as Harvester. Inspect PR checks and review threads. Resolve actionable feedback and merge only when required checks and conversations are green. Preserve valuable unfinished work.',
    capabilities: ['review', 'fix', 'merge'],
    triggers: [{ type: 'schedule', value: '*/15 * * * *' }],
  },
  curator: {
    name: 'curator',
    description: 'Assess issues, prioritize work, and coordinate agents.',
    systemPrompt:
      'Act as Curator. Assess issues against current repository evidence, prioritize actionable work, and delegate clear tasks using existing agent tools. Avoid duplicate issues and competing writers.',
    capabilities: ['triage', 'delegate'],
    triggers: [{ type: 'schedule', value: '0 2 * * *' }],
  },
  reviewer: {
    name: 'reviewer',
    description: 'Review code for correctness and maintainability.',
    systemPrompt:
      'Act as Reviewer. Inspect the diff and relevant context. Report concrete correctness, security, and maintainability findings with file references. Explain evidence and avoid speculative findings.',
    capabilities: ['review'],
    triggers: [
      { type: 'event', value: 'pull_request' },
      { type: 'command', value: '/review' },
    ],
  },
  fixer: {
    name: 'fixer',
    description: 'Diagnose CI failures and verify targeted repairs.',
    systemPrompt:
      'Act as Fixer. Read failing CI logs, reproduce the failure, and repair the root cause with forward commits. Never weaken tests or required checks. Verify the repair before reporting success.',
    capabilities: ['fix'],
    triggers: [{ type: 'event', value: 'workflow_run:failure' }],
  },
  delegator: {
    name: 'delegator',
    description: 'Route issues to suitable agents with explicit ownership.',
    systemPrompt:
      'Act as Delegator. Route the issue to a suitable existing agent with a clear goal, owned files, acceptance criteria, and repository context. Check for existing assignments before spawning work.',
    capabilities: ['delegate'],
    triggers: [
      { type: 'command', value: '/cursor' },
      { type: 'command', value: '/jules' },
    ],
  },
};

// Protect the shared defaults from mutation by JavaScript consumers.
for (const definition of Object.values(DEFAULT_ROLES)) {
  for (const trigger of definition.triggers) Object.freeze(trigger);
  Object.freeze(definition.triggers);
  Object.freeze(definition.capabilities);
  Object.freeze(definition);
}
Object.freeze(DEFAULT_ROLES);
