import { beforeEach, describe, expect, it, vi } from 'vitest';

const { getTokenForRepoMock, spawnSyncMock } = vi.hoisted(() => ({
  getTokenForRepoMock: vi.fn(),
  spawnSyncMock: vi.fn(),
}));

vi.mock('node:child_process', () => ({
  spawnSync: spawnSyncMock,
}));

vi.mock('../src/core/tokens.js', async () => {
  const actual = await vi.importActual<typeof import('../src/core/tokens.js')>(
    '../src/core/tokens.js'
  );
  return {
    ...actual,
    getTokenForRepo: getTokenForRepoMock,
  };
});

import { cloneRepo } from '../src/github/client.js';

describe('cloneRepo', () => {
  beforeEach(() => {
    getTokenForRepoMock.mockReset();
    spawnSyncMock.mockReset();

    getTokenForRepoMock.mockReturnValue('ghp_test_token');
    spawnSyncMock.mockReturnValue({
      status: 0,
      stderr: '',
      stdout: '',
      error: undefined,
    });
  });

  it('tokenizes bare owner/repo clone targets', () => {
    const result = cloneRepo('owner/repo', '/tmp/repo');

    expect(result).toEqual({ success: true });
    expect(spawnSyncMock).toHaveBeenCalledWith(
      'git',
      ['clone', 'https://oauth2:ghp_test_token@github.com/owner/repo.git', '/tmp/repo'],
      {
        encoding: 'utf-8',
        stdio: 'pipe',
        timeout: 120000,
      }
    );
  });

  it('tokenizes https GitHub clone targets', () => {
    const result = cloneRepo('https://github.com/owner/repo.git', '/tmp/repo');

    expect(result).toEqual({ success: true });
    expect(spawnSyncMock).toHaveBeenCalledWith(
      'git',
      ['clone', 'https://oauth2:ghp_test_token@github.com/owner/repo.git', '/tmp/repo'],
      {
        encoding: 'utf-8',
        stdio: 'pipe',
        timeout: 120000,
      }
    );
  });

  it('rewrites GitHub ssh clone targets to tokenized https URLs', () => {
    const result = cloneRepo('git@github.com:owner/repo.git', '/tmp/repo');

    expect(result).toEqual({ success: true });
    expect(spawnSyncMock).toHaveBeenCalledWith(
      'git',
      ['clone', 'https://oauth2:ghp_test_token@github.com/owner/repo.git', '/tmp/repo'],
      {
        encoding: 'utf-8',
        stdio: 'pipe',
        timeout: 120000,
      }
    );
  });

  it('redacts embedded tokens from git stderr on clone failure', () => {
    spawnSyncMock.mockReturnValue({
      status: 1,
      stderr: 'fatal: could not read https://oauth2:ghp_secret@github.com/owner/repo.git',
      stdout: '',
      error: undefined,
    });

    const result = cloneRepo('owner/repo', '/tmp/repo');

    expect(result.success).toBe(false);
    expect(result.error).toContain('oauth2:[REDACTED]@github.com');
    expect(result.error).not.toContain('ghp_secret');
  });

  it('returns a token error before invoking git when no repo token is available', () => {
    getTokenForRepoMock.mockReturnValue(undefined);

    const result = cloneRepo('owner/repo', '/tmp/repo');

    expect(result).toEqual({
      success: false,
      error: 'No token available for repo: owner/repo',
    });
    expect(spawnSyncMock).not.toHaveBeenCalled();
  });
});
