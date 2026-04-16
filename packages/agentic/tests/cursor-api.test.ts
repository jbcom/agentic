/**
 * Tests for CursorAPI - input validation, SSRF protection, and request handling
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { CursorAPI } from '../src/fleet/cursor-api.js';

describe('CursorAPI', () => {
  const originalFetch = global.fetch;

  beforeEach(() => {
    process.env.CURSOR_API_KEY = 'test-key';
    global.fetch = vi.fn();
  });

  afterEach(() => {
    global.fetch = originalFetch;
    delete process.env.CURSOR_API_KEY;
    vi.restoreAllMocks();
  });

  describe('Constructor and Configuration', () => {
    it('should throw when no API key is available', () => {
      delete process.env.CURSOR_API_KEY;
      expect(() => new CursorAPI()).toThrow('CURSOR_API_KEY is required');
    });

    it('should accept API key from constructor options', () => {
      delete process.env.CURSOR_API_KEY;
      const api = new CursorAPI({ apiKey: 'explicit-key' });
      expect(api).toBeDefined();
    });

    it('should use CURSOR_API_KEY from environment', () => {
      const api = new CursorAPI();
      expect(api).toBeDefined();
    });

    it('should check static availability', () => {
      expect(CursorAPI.isAvailable()).toBe(true);
      delete process.env.CURSOR_API_KEY;
      expect(CursorAPI.isAvailable()).toBe(false);
    });
  });

  describe('Agent ID Validation', () => {
    it('should reject empty agent ID', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(api.getAgentStatus('')).rejects.toThrow('Agent ID is required');
    });

    it('should reject agent ID exceeding max length', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      const longId = 'a'.repeat(101);
      await expect(api.getAgentStatus(longId)).rejects.toThrow('exceeds maximum length');
    });

    it('should reject agent ID with invalid characters', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(api.getAgentStatus('bc-123; rm -rf /')).rejects.toThrow(
        'invalid characters'
      );
    });

    it('should accept valid agent ID with alphanumeric and hyphens', async () => {
      const api = new CursorAPI({ apiKey: 'test-key', retryDelay: 1 });

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () =>
          Promise.resolve(
            JSON.stringify({ id: 'bc-valid-123', status: 'RUNNING' })
          ),
      });

      const result = await api.getAgentStatus('bc-valid-123');
      expect(result.success).toBe(true);
    });
  });

  describe('Prompt Validation', () => {
    it('should reject empty prompt text', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(
        api.launchAgent({
          prompt: { text: '' },
          source: { repository: 'owner/repo' },
        })
      ).rejects.toThrow('Prompt text');
    });

    it('should reject whitespace-only prompt text', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(
        api.launchAgent({
          prompt: { text: '   ' },
          source: { repository: 'owner/repo' },
        })
      ).rejects.toThrow('Prompt text cannot be empty');
    });

    it('should reject prompt exceeding max length', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      const longPrompt = 'x'.repeat(100001);
      await expect(
        api.launchAgent({
          prompt: { text: longPrompt },
          source: { repository: 'owner/repo' },
        })
      ).rejects.toThrow('exceeds maximum length');
    });
  });

  describe('Repository Validation', () => {
    it('should reject repository without slash', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(
        api.launchAgent({
          prompt: { text: 'test task' },
          source: { repository: 'noslash' },
        })
      ).rejects.toThrow("must be in format 'owner/repo'");
    });

    it('should reject repository exceeding max length', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      const longRepo = `${'a'.repeat(100)}/${'b'.repeat(101)}`;
      await expect(
        api.launchAgent({
          prompt: { text: 'test task' },
          source: { repository: longRepo },
        })
      ).rejects.toThrow('exceeds maximum length');
    });
  });

  describe('Webhook URL Validation (SSRF Protection)', () => {
    it('should reject HTTP webhook URLs', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(
        api.launchAgent({
          prompt: { text: 'test' },
          source: { repository: 'owner/repo' },
          webhook: { url: 'http://example.com/hook' },
        })
      ).rejects.toThrow('HTTPS protocol');
    });

    it('should reject localhost webhook URLs', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(
        api.launchAgent({
          prompt: { text: 'test' },
          source: { repository: 'owner/repo' },
          webhook: { url: 'https://localhost/hook' },
        })
      ).rejects.toThrow('internal/private');
    });

    it('should reject private IP webhook URLs', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      for (const ip of ['10.0.0.1', '192.168.1.1', '172.16.0.1']) {
        await expect(
          api.launchAgent({
            prompt: { text: 'test' },
            source: { repository: 'owner/repo' },
            webhook: { url: `https://${ip}/hook` },
          })
        ).rejects.toThrow('internal/private');
      }
    });

    it('should reject cloud metadata endpoint URLs', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });
      await expect(
        api.launchAgent({
          prompt: { text: 'test' },
          source: { repository: 'owner/repo' },
          webhook: { url: 'https://169.254.169.254/latest/meta-data/' },
        })
      ).rejects.toThrow('internal/private');
    });

    it('should accept valid HTTPS webhook URLs', async () => {
      const api = new CursorAPI({ apiKey: 'test-key', retryDelay: 1 });

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () =>
          Promise.resolve(JSON.stringify({ id: 'bc-1', status: 'RUNNING' })),
      });

      const result = await api.launchAgent({
        prompt: { text: 'test task' },
        source: { repository: 'owner/repo' },
        webhook: { url: 'https://hooks.example.com/notify' },
      });

      expect(result.success).toBe(true);
    });
  });

  describe('Followup Validation', () => {
    it('should validate both agent ID and prompt for followup', async () => {
      const api = new CursorAPI({ apiKey: 'test-key' });

      await expect(api.addFollowup('', { text: 'hello' })).rejects.toThrow(
        'Agent ID is required'
      );

      await expect(api.addFollowup('bc-123', { text: '' })).rejects.toThrow(
        'Prompt text'
      );
    });
  });

  describe('Response Handling', () => {
    it('should handle empty JSON response gracefully', async () => {
      const api = new CursorAPI({ apiKey: 'test-key', retryDelay: 1 });

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
        ok: true,
        status: 204,
        headers: { get: () => null },
        text: () => Promise.resolve(''),
      });

      const result = await api.listAgents();
      expect(result.success).toBe(true);
    });

    it('should handle invalid JSON response', async () => {
      const api = new CursorAPI({ apiKey: 'test-key', retryDelay: 1 });

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve('not json at all'),
      });

      const result = await api.listAgents();
      expect(result.success).toBe(false);
      expect(result.error).toContain('Invalid JSON');
    });
  });
});
