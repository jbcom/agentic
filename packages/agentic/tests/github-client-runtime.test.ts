import { beforeEach, describe, expect, it, vi } from 'vitest';

const {
  checksListForRefMock,
  getCombinedStatusForRefMock,
  listIssueCommentsMock,
  listReviewCommentsMock,
  listReviewsMock,
  octokitConstructorMock,
  pullsGetMock,
} = vi.hoisted(() => {
  const pullsGet = vi.fn();
  const listReviews = vi.fn();
  const listReviewComments = vi.fn();
  const listIssueComments = vi.fn();
  const checksListForRef = vi.fn();
  const getCombinedStatusForRef = vi.fn();
  const octokitInstance = {
    pulls: {
      get: pullsGet,
      listReviewComments,
      listReviews,
    },
    issues: {
      listComments: listIssueComments,
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
    listIssueCommentsMock: listIssueComments,
    listReviewCommentsMock: listReviewComments,
    listReviewsMock: listReviews,
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
    listIssueCommentsMock.mockReset();
    listReviewsMock.mockReset();
    listReviewCommentsMock.mockReset();

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
    listReviewsMock.mockResolvedValue({
      data: [],
    });
    listIssueCommentsMock.mockResolvedValue({
      data: [],
    });
    listReviewCommentsMock.mockResolvedValue({
      data: [],
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

  it('treats changes-requested review summaries as unaddressed feedback', async () => {
    listReviewsMock.mockResolvedValue({
      data: [
        {
          id: 17,
          state: 'CHANGES_REQUESTED',
          body: 'Critical: please fix the failing migration path.',
          submitted_at: '2026-04-15T13:00:00Z',
          html_url: 'https://github.com/owner/repo/pull/42#pullrequestreview-17',
          user: { login: 'reviewer-1' },
        },
      ],
    });

    const client = new GitHubClient({
      token: 'ghp_test_review_feedback',
      owner: 'owner',
      repo: 'repo',
    });

    const feedback = await client.collectFeedback(42);

    expect(feedback).toEqual([
      {
        id: 'review-17',
        author: 'reviewer-1',
        body: 'Critical: please fix the failing migration path.',
        path: null,
        line: null,
        severity: 'critical',
        status: 'unaddressed',
        createdAt: '2026-04-15T13:00:00Z',
        url: 'https://github.com/owner/repo/pull/42#pullrequestreview-17',
        isAutoResolvable: false,
        suggestedAction: null,
        resolution: null,
      },
    ]);
  });

  it('treats approved and dismissed review summaries as non-blocking feedback states', async () => {
    listReviewsMock.mockResolvedValue({
      data: [
        {
          id: 21,
          state: 'APPROVED',
          body: 'Looks good to me.',
          submitted_at: '2026-04-15T13:10:00Z',
          html_url: 'https://github.com/owner/repo/pull/42#pullrequestreview-21',
          user: { login: 'reviewer-2' },
        },
        {
          id: 22,
          state: 'DISMISSED',
          body: 'Superseded by a newer review.',
          submitted_at: '2026-04-15T13:12:00Z',
          html_url: 'https://github.com/owner/repo/pull/42#pullrequestreview-22',
          user: { login: 'reviewer-3' },
        },
      ],
    });

    const client = new GitHubClient({
      token: 'ghp_test_review_states',
      owner: 'owner',
      repo: 'repo',
    });

    const feedback = await client.collectFeedback(42);

    expect(feedback.map((item) => ({ id: item.id, status: item.status }))).toEqual([
      {
        id: 'review-21',
        status: 'addressed',
      },
      {
        id: 'review-22',
        status: 'dismissed',
      },
    ]);
  });

  it('includes top-level PR conversation comments in collected feedback', async () => {
    listIssueCommentsMock.mockResolvedValue({
      data: [
        {
          id: 31,
          body: 'Please fix the release notes before merging.',
          created_at: '2026-04-15T13:20:00Z',
          html_url: 'https://github.com/owner/repo/pull/42#issuecomment-31',
          user: { login: 'maintainer-1' },
        },
      ],
    });

    const client = new GitHubClient({
      token: 'ghp_test_issue_comment_feedback',
      owner: 'owner',
      repo: 'repo',
    });

    const feedback = await client.collectFeedback(42);

    expect(feedback).toEqual([
      {
        id: 'issue-31',
        author: 'maintainer-1',
        body: 'Please fix the release notes before merging.',
        path: null,
        line: null,
        severity: 'high',
        status: 'unaddressed',
        createdAt: '2026-04-15T13:20:00Z',
        url: 'https://github.com/owner/repo/pull/42#issuecomment-31',
        isAutoResolvable: false,
        suggestedAction: null,
        resolution: null,
      },
    ]);
  });
});
