import { describe, expect, it } from 'vitest';
import { resolveSuccessorAgentId } from '../src/handoff/cli.js';

describe('handoff CLI helpers', () => {
  it('prefers an explicit successor id over the environment', () => {
    const result = resolveSuccessorAgentId('bc-explicit', {
      CURSOR_AGENT_ID: 'bc-env',
    });

    expect(result).toEqual({
      success: true,
      data: 'bc-explicit',
    });
  });

  it('uses CURSOR_AGENT_ID when no explicit successor id is provided', () => {
    const result = resolveSuccessorAgentId(undefined, {
      CURSOR_AGENT_ID: 'bc-env',
    });

    expect(result).toEqual({
      success: true,
      data: 'bc-env',
    });
  });

  it('fails when no successor id is available', () => {
    const result = resolveSuccessorAgentId(undefined, {});

    expect(result).toEqual({
      success: false,
      error: 'Successor agent ID is required. Pass --successor-id or set CURSOR_AGENT_ID.',
    });
  });

  it('rejects invalid successor agent ids', () => {
    const result = resolveSuccessorAgentId('bc invalid', {});

    expect(result).toEqual({
      success: false,
      error: 'Successor agent ID contains invalid characters.',
    });
  });
});
