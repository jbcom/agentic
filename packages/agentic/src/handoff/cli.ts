import type { Result } from '../core/types.js';

const AGENT_ID_PATTERN = /^[a-zA-Z0-9-]+$/;
const MAX_AGENT_ID_LENGTH = 100;

export function resolveSuccessorAgentId(
  explicitSuccessorId?: string,
  env: NodeJS.ProcessEnv = process.env
): Result<string> {
  const successorId = explicitSuccessorId?.trim() || env.CURSOR_AGENT_ID?.trim();

  if (!successorId) {
    return {
      success: false,
      error: 'Successor agent ID is required. Pass --successor-id or set CURSOR_AGENT_ID.',
    };
  }

  if (successorId.length > MAX_AGENT_ID_LENGTH) {
    return {
      success: false,
      error: `Successor agent ID exceeds maximum length (${MAX_AGENT_ID_LENGTH} characters).`,
    };
  }

  if (!AGENT_ID_PATTERN.test(successorId)) {
    return {
      success: false,
      error: 'Successor agent ID contains invalid characters.',
    };
  }

  return {
    success: true,
    data: successorId,
  };
}
