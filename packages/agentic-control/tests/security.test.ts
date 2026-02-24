/**
 * Tests for security utilities - token sanitization, safe error handling,
 * and environment sanitization
 */

import { describe, expect, it, vi } from 'vitest';
import {
  createSafeError,
  sanitizeEnvironment,
  sanitizeError,
  safeConsole,
} from '../src/core/security.js';

describe('Token Sanitization', () => {
  describe('sanitizeError', () => {
    it('should redact GitHub personal access tokens (ghp_)', () => {
      const message =
        'Auth failed with ghp_1234567890abcdef1234567890abcdef12345678';
      const sanitized = sanitizeError(message);
      expect(sanitized).not.toContain('ghp_');
      expect(sanitized).toContain('[REDACTED_TOKEN]');
    });

    it('should redact GitHub OAuth tokens (gho_)', () => {
      const message =
        'OAuth error: gho_1234567890abcdef1234567890abcdef12345678';
      const sanitized = sanitizeError(message);
      expect(sanitized).not.toContain('gho_');
      expect(sanitized).toContain('[REDACTED_TOKEN]');
    });

    it('should redact Anthropic API keys (sk-ant-)', () => {
      const key =
        'sk-ant-api03-' + 'x'.repeat(95);
      const message = `API call failed: ${key}`;
      const sanitized = sanitizeError(message);
      expect(sanitized).not.toContain('sk-ant-');
      expect(sanitized).toContain('[REDACTED_TOKEN]');
    });

    it('should redact OpenAI API keys (sk-)', () => {
      const key = 'sk-' + 'x'.repeat(48);
      const message = `OpenAI error: ${key}`;
      const sanitized = sanitizeError(message);
      expect(sanitized).not.toContain(key);
      expect(sanitized).toContain('[REDACTED_TOKEN]');
    });

    it('should handle Error objects', () => {
      const error = new Error(
        'Failed with ghp_1234567890abcdef1234567890abcdef12345678'
      );
      const sanitized = sanitizeError(error);
      expect(sanitized).not.toContain('ghp_');
      expect(sanitized).toContain('[REDACTED_TOKEN]');
    });

    it('should return original message when no tokens present', () => {
      const message = 'Simple error with no tokens';
      const sanitized = sanitizeError(message);
      expect(sanitized).toBe(message);
    });

    it('should handle multiple tokens in one message', () => {
      const message =
        'Token1: ghp_abcdefghijklmnopqrstuvwxyz1234567890ab Token2: ghp_zyxwvutsrqponmlkjihgfedcba0987654321ab';
      const sanitized = sanitizeError(message);
      expect(sanitized).not.toContain('ghp_');
      // Both should be redacted
      expect(sanitized.match(/\[REDACTED_TOKEN\]/g)?.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe('sanitizeEnvironment', () => {
    it('should redact values for token-related env vars', () => {
      const env = {
        GITHUB_TOKEN: 'ghp_secret',
        API_KEY: 'sk-secret',
        SECRET_VALUE: 'very-secret',
      };

      const sanitized = sanitizeEnvironment(env);
      expect(sanitized.GITHUB_TOKEN).toBe('[REDACTED]');
      expect(sanitized.API_KEY).toBe('[REDACTED]');
      expect(sanitized.SECRET_VALUE).toBe('[REDACTED]');
    });

    it('should preserve non-sensitive env vars', () => {
      const env = {
        NODE_ENV: 'production',
        LOG_LEVEL: 'info',
        PORT: '3000',
      };

      const sanitized = sanitizeEnvironment(env);
      expect(sanitized.NODE_ENV).toBe('production');
      expect(sanitized.LOG_LEVEL).toBe('info');
      expect(sanitized.PORT).toBe('3000');
    });

    it('should show (not set) for empty sensitive vars', () => {
      const env = {
        GITHUB_TOKEN: '',
        NORMAL_VAR: 'value',
      };

      const sanitized = sanitizeEnvironment(env);
      expect(sanitized.GITHUB_TOKEN).toBe('(not set)');
    });

    it('should handle undefined values', () => {
      const env: Record<string, string | undefined> = {
        SOME_VAR: undefined,
      };

      const sanitized = sanitizeEnvironment(env);
      expect(sanitized.SOME_VAR).toBe('undefined');
    });

    it('should be case-insensitive for sensitive var detection', () => {
      const env = {
        my_Token_var: 'secret1',
        API_KEY_custom: 'secret2',
        some_secret: 'secret3',
      };

      const sanitized = sanitizeEnvironment(env);
      expect(sanitized.my_Token_var).toBe('[REDACTED]');
      expect(sanitized.API_KEY_custom).toBe('[REDACTED]');
      expect(sanitized.some_secret).toBe('[REDACTED]');
    });
  });

  describe('createSafeError', () => {
    it('should sanitize the error message', () => {
      const error = createSafeError(
        'Failed with key ghp_1234567890abcdef1234567890abcdef12345678'
      );
      expect(error.message).not.toContain('ghp_');
      expect(error.message).toContain('[REDACTED_TOKEN]');
    });

    it('should sanitize the cause error', () => {
      const originalError = new Error(
        'Caused by ghp_1234567890abcdef1234567890abcdef12345678'
      );
      const error = createSafeError('Wrapper error', originalError);

      expect(error.cause).toBeInstanceOf(Error);
      expect((error.cause as Error).message).not.toContain('ghp_');
    });

    it('should sanitize the stack trace', () => {
      const originalError = new Error('original');
      // Inject a fake stack with a token
      originalError.stack =
        'Error at ghp_1234567890abcdef1234567890abcdef12345678:line:1';
      const error = createSafeError('wrapper', originalError);

      expect(error.stack).toBeDefined();
      expect(error.stack).not.toContain('ghp_');
    });

    it('should handle missing original error', () => {
      const error = createSafeError('Simple error');
      expect(error).toBeInstanceOf(Error);
      expect(error.cause).toBeUndefined();
    });
  });

  describe('safeConsole', () => {
    it('should sanitize log messages', () => {
      const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
      safeConsole.log(
        'Token: ghp_1234567890abcdef1234567890abcdef12345678'
      );
      expect(consoleSpy).toHaveBeenCalled();
      const loggedMessage = consoleSpy.mock.calls[0][0];
      expect(loggedMessage).not.toContain('ghp_');
      consoleSpy.mockRestore();
    });

    it('should sanitize error messages', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      safeConsole.error(
        'Error with ghp_1234567890abcdef1234567890abcdef12345678'
      );
      const loggedMessage = consoleSpy.mock.calls[0][0];
      expect(loggedMessage).not.toContain('ghp_');
      consoleSpy.mockRestore();
    });

    it('should sanitize warn messages', () => {
      const consoleSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
      safeConsole.warn(
        'Warning with ghp_1234567890abcdef1234567890abcdef12345678'
      );
      const loggedMessage = consoleSpy.mock.calls[0][0];
      expect(loggedMessage).not.toContain('ghp_');
      consoleSpy.mockRestore();
    });

    it('should sanitize string args but pass through non-strings', () => {
      const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
      const objectArg = { key: 'value' };
      safeConsole.log('message', 'ghp_1234567890abcdef1234567890abcdef12345678', objectArg);
      const secondArg = consoleSpy.mock.calls[0][1];
      expect(secondArg).not.toContain('ghp_');
      const thirdArg = consoleSpy.mock.calls[0][2];
      expect(thirdArg).toBe(objectArg);
      consoleSpy.mockRestore();
    });
  });
});
