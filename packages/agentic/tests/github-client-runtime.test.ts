import { beforeEach, describe, expect, it, vi } from 'vitest';

const {
  checksListForRefMock,
  getCombinedStatusForRefMock,
  octokitConstructorMock,
  pullsGetMock,
} = vi.hoisted(() => {
  const pullsGet = vi.fn();
  const checksListForRef = vi.fn();
  const getCombinedStatusForRef = vi.fn();
  const octokitInstance = {
    pulls: {
      get: pullsGet,
    },
    checks: {
      listForRef: checksListForRef,
    },
    repos: {
      getCombinedStatusForRef,
    },
  };

  return {
    checksListForRefMock: checksListForRef,
    getCombinedStatusForRefMock: getCombinedStatusForRef,
    octokitConstructorMock: vi.fn(function MockOctokit() {
      return octokitInstance;
    }),
    pullsGetMock: pullsGet,
  };
});

vi.mock('@octokit/rest', () => ({
  Octokit: octokitConstructorMock,
}));

import { GitHubClient } from '../src/github/client.js';

describe('GitHubClient runtime behavior', () => {
  beforeEach(() => {
    pullsGetMock.mockReset();
    checksListForRefMock.mockReset();
    getCombinedStatusForRefMock.mockReset();

    pullsGetMock.mockResolvedValue({
      data: {
        head: {
          sha: 'abc123',
        },
      },
    });
    checksListForRefMock.mockResolvedValue({
      data: {
        check_runs: [],
      },
    });
    getCombinedStatusForRefMock.mockResolvedValue({
      data: {
        state: 'success',
        statuses: [],
      },
    });
  });

  it('treats classic pending commit statuses as pending CI even when no check runs exist', async () => {
    getCombinedStatusForRefMock.mockResolvedValue({
      data: {
        state: 'pending',
        statuses: [
          {
            context: 'ci/legacy-build',
            state: 'pending',
            target_url: 'https://ci.example.test/build/1',
            created_at: '2026-04-15T12:00:00Z',
            updated_at: '2026-04-15T12:05:00Z',
          },
        ],
      },
    });

    const client = new GitHubClient({
      token: 'ghp_test_pending_status',
      owner: 'owner',
      repo: 'repo',
    });

    const result = await client.getCIStatus(42);

    expect(result.allPassing).toBe(false);
    expect(result.anyPending).toBe(true);
    expect(result.failures).toEqual([]);
    expect(result.checks).toEqual([
      {
        name: 'ci/legacy-build',
        status: 'pending',
        conclusion: 'pending',
        url: 'https://ci.example.test/build/1',
        startedAt: '2026-04-15T12:00:00Z',
        completedAt: '2026-04-15T12:05:00Z',
      },
    ]);
  });

  it('treats classic error and failure commit statuses as CI failures', async () => {
    getCombinedStatusForRefMock.mockResolvedValue({
      data: {
        state: 'failure',
        statuses: [
          {
            context: 'ci/external',
            state: 'error',
            target_url: 'https://ci.example.test/build/2',
            created_at: '2026-04-15T12:10:00Z',
            updated_at: '2026-04-15T12:20:00Z',
          },
        ],
      },
    });

    const client = new GitHubClient({
      token: 'ghp_test_failure_status',
      owner: 'owner',
      repo: 'repo',
    });

    const result = await client.getCIStatus(42);

    expect(result.allPassing).toBe(false);
    expect(result.anyPending).toBe(false);
    expect(result.failures).toEqual([
      {
        name: 'ci/external',
        status: 'failure',
        conclusion: 'error',
        url: 'https://ci.example.test/build/2',
        startedAt: '2026-04-15T12:10:00Z',
        completedAt: '2026-04-15T12:20:00Z',
      },
    ]);
  });
});
