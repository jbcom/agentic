export { DEFAULT_ROLES } from './defaults.js';
export { executeRole, findRoleByTrigger, getEffectiveRole, listRoles } from './executor.js';
export type {
  AgentCapability,
  EffectiveRole,
  RoleConfig,
  RoleDefinition,
  RoleExecutionOptions,
  RoleName,
  RolesConfig,
  RoleTrigger,
} from './types.js';
export { ROLE_NAMES } from './types.js';
