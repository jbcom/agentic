import { beforeEach, describe, expect, it, vi } from 'vitest';

const {
  createCommentMock,
  getPRReviewTokenMock,
  getTokenForRepoMock,
  listCommentsMock,
  octokitConstructorMock,
  requestReviewersMock,
} = vi.hoisted(() => {
  const listComments = vi.fn();
  const createComment = vi.fn();
  const requestReviewers = vi.fn();
  const octokitInstance = {
    issues: {
      listComments,
      createComment,
    },
    pulls: {
      requestReviewers,
    },
  };

  return {
    createCommentMock: createComment,
    getPRReviewTokenMock: vi.fn(),
    getTokenForRepoMock: vi.fn(),
    listCommentsMock: listComments,
    octokitConstructorMock: vi.fn(function MockOctokit() {
      return octokitInstance;
    }),
    requestReviewersMock: requestReviewers,
  };
});

vi.mock('@octokit/rest', () => ({
  Octokit: octokitConstructorMock,
}));

vi.mock('../src/core/tokens.js', async () => {
  const actual = await vi.importActual<typeof import('../src/core/tokens.js')>(
    '../src/core/tokens.js'
  );
  return {
    ...actual,
    getPRReviewToken: getPRReviewTokenMock,
    getTokenForRepo: getTokenForRepoMock,
  };
});

import { GitHubClient } from '../src/github/client.js';

describe('GitHubClient static operations', () => {
  beforeEach(() => {
    getTokenForRepoMock.mockReset();
    getPRReviewTokenMock.mockReset();
    listCommentsMock.mockReset();
    createCommentMock.mockReset();
    requestReviewersMock.mockReset();

    getTokenForRepoMock.mockReturnValue('ghp_repo_token');
    getPRReviewTokenMock.mockReturnValue('ghp_review_token');
    createCommentMock.mockResolvedValue({
      data: {
        id: 77,
        body: 'Acknowledged',
        created_at: '2026-04-15T18:00:00Z',
        updated_at: '2026-04-15T18:00:00Z',
        user: { login: 'review-bot' },
      },
    });
    requestReviewersMock.mockResolvedValue({});
  });

  it('paginates PR comments beyond the first 100 issue comments', async () => {
    listCommentsMock
      .mockResolvedValueOnce({
        data: Array.from({ length: 100 }, (_, index) => ({
          id: index + 1,
          body: `Comment ${index + 1}`,
          user: { login: `user-${index + 1}` },
          created_at: `2026-04-15T18:${String(index).padStart(2, '0')}:00Z`,
          updated_at: `2026-04-15T18:${String(index).padStart(2, '0')}:30Z`,
        })),
      })
      .mockResolvedValueOnce({
        data: [
          {
            id: 101,
            body: '@cursor latest coordination update',
            user: { login: 'maintainer-1' },
            created_at: '2026-04-15T19:40:00Z',
            updated_at: '2026-04-15T19:40:30Z',
          },
          {
            id: 102,
            body: 'Final follow-up',
            user: { login: 'maintainer-2' },
            created_at: '2026-04-15T19:41:00Z',
            updated_at: '2026-04-15T19:41:30Z',
          },
        ],
      });

    const result = await GitHubClient.listPRComments('owner', 'repo', 42);

    expect(result.success).toBe(true);
    expect(result.data).toHaveLength(102);
    expect(result.data?.[100]).toEqual({
      id: 101,
      body: '@cursor latest coordination update',
      author: 'maintainer-1',
      createdAt: '2026-04-15T19:40:00Z',
      updatedAt: '2026-04-15T19:40:30Z',
    });
    expect(listCommentsMock).toHaveBeenNthCalledWith(1, {
      owner: 'owner',
      repo: 'repo',
      issue_number: 42,
      per_page: 100,
      page: 1,
    });
    expect(listCommentsMock).toHaveBeenNthCalledWith(2, {
      owner: 'owner',
      repo: 'repo',
      issue_number: 42,
      per_page: 100,
      page: 2,
    });
  });
});
