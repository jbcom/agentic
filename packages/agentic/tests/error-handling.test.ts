/**
 * Tests for typed error classes and error hierarchy
 */

import { describe, expect, it } from 'vitest';
import {
  ConfigErrorCode,
  ConfigurationError,
  DockerBuildError,
  DockerErrorCode,
  SandboxError,
  SandboxErrorCode,
} from '../src/core/errors.js';

describe('SandboxError', () => {
  it('should create error with code and containerId', () => {
    const error = new SandboxError(
      'Container failed',
      SandboxErrorCode.CONTAINER_START_FAILED,
      'abc-123'
    );

    expect(error).toBeInstanceOf(Error);
    expect(error).toBeInstanceOf(SandboxError);
    expect(error.name).toBe('SandboxError');
    expect(error.message).toBe('Container failed');
    expect(error.code).toBe(SandboxErrorCode.CONTAINER_START_FAILED);
    expect(error.containerId).toBe('abc-123');
  });

  it('should create error without containerId', () => {
    const error = new SandboxError(
      'Runtime not found',
      SandboxErrorCode.RUNTIME_NOT_FOUND
    );

    expect(error.containerId).toBeUndefined();
    expect(error.code).toBe('RUNTIME_NOT_FOUND');
  });

  it('should preserve cause error', () => {
    const cause = new Error('underlying issue');
    const error = new SandboxError(
      'Wrapper',
      SandboxErrorCode.EXECUTION_TIMEOUT,
      'container-1',
      cause
    );

    expect(error.cause).toBe(cause);
  });

  it('should have all expected error codes', () => {
    const expectedCodes = [
      'CONTAINER_CREATE_FAILED',
      'CONTAINER_START_FAILED',
      'EXECUTION_TIMEOUT',
      'MEMORY_LIMIT_EXCEEDED',
      'WORKSPACE_MOUNT_FAILED',
      'OUTPUT_EXTRACTION_FAILED',
      'RUNTIME_NOT_FOUND',
    ];

    for (const code of expectedCodes) {
      expect(Object.values(SandboxErrorCode)).toContain(code);
    }
  });

  it('should be catchable as Error', () => {
    try {
      throw new SandboxError('test', SandboxErrorCode.EXECUTION_TIMEOUT);
    } catch (e) {
      expect(e).toBeInstanceOf(Error);
      expect((e as SandboxError).code).toBe(SandboxErrorCode.EXECUTION_TIMEOUT);
    }
  });
});

describe('DockerBuildError', () => {
  it('should create error with code and dockerfile', () => {
    const error = new DockerBuildError(
      'Build failed',
      DockerErrorCode.BUILD_FAILED,
      'Dockerfile.prod'
    );

    expect(error).toBeInstanceOf(Error);
    expect(error).toBeInstanceOf(DockerBuildError);
    expect(error.name).toBe('DockerBuildError');
    expect(error.message).toBe('Build failed');
    expect(error.code).toBe(DockerErrorCode.BUILD_FAILED);
    expect(error.dockerfile).toBe('Dockerfile.prod');
  });

  it('should create error without dockerfile', () => {
    const error = new DockerBuildError(
      'Push error',
      DockerErrorCode.PUSH_FAILED
    );

    expect(error.dockerfile).toBeUndefined();
  });

  it('should preserve cause error', () => {
    const cause = new Error('network issue');
    const error = new DockerBuildError(
      'Registry auth failed',
      DockerErrorCode.REGISTRY_AUTH_FAILED,
      undefined,
      cause
    );

    expect(error.cause).toBe(cause);
  });

  it('should have all expected error codes', () => {
    const expectedCodes = [
      'BUILD_FAILED',
      'PUSH_FAILED',
      'PLATFORM_NOT_SUPPORTED',
      'REGISTRY_AUTH_FAILED',
    ];

    for (const code of expectedCodes) {
      expect(Object.values(DockerErrorCode)).toContain(code);
    }
  });
});

describe('ConfigurationError', () => {
  it('should create error with code and field', () => {
    const error = new ConfigurationError(
      'Invalid configuration',
      ConfigErrorCode.INVALID_SCHEMA,
      'triage.model'
    );

    expect(error).toBeInstanceOf(Error);
    expect(error).toBeInstanceOf(ConfigurationError);
    expect(error.name).toBe('ConfigurationError');
    expect(error.message).toBe('Invalid configuration');
    expect(error.code).toBe(ConfigErrorCode.INVALID_SCHEMA);
    expect(error.field).toBe('triage.model');
  });

  it('should create error without field', () => {
    const error = new ConfigurationError(
      'File not found',
      ConfigErrorCode.FILE_NOT_FOUND
    );

    expect(error.field).toBeUndefined();
  });

  it('should support missing required field code', () => {
    const error = new ConfigurationError(
      'Token env var missing',
      ConfigErrorCode.MISSING_REQUIRED_FIELD,
      'tokens.defaultTokenEnvVar'
    );

    expect(error.code).toBe('MISSING_REQUIRED_FIELD');
    expect(error.field).toBe('tokens.defaultTokenEnvVar');
  });

  it('should have all expected error codes', () => {
    const expectedCodes = [
      'INVALID_SCHEMA',
      'MISSING_REQUIRED_FIELD',
      'INVALID_VALUE',
      'FILE_NOT_FOUND',
    ];

    for (const code of expectedCodes) {
      expect(Object.values(ConfigErrorCode)).toContain(code);
    }
  });

  it('should preserve cause error', () => {
    const cause = new Error('parse error');
    const error = new ConfigurationError(
      'Invalid value',
      ConfigErrorCode.INVALID_VALUE,
      'logLevel',
      cause
    );

    expect(error.cause).toBe(cause);
  });
});

describe('Error Hierarchy and instanceof', () => {
  it('should distinguish between error types', () => {
    const sandbox = new SandboxError('s', SandboxErrorCode.EXECUTION_TIMEOUT);
    const docker = new DockerBuildError('d', DockerErrorCode.BUILD_FAILED);
    const config = new ConfigurationError('c', ConfigErrorCode.INVALID_SCHEMA);

    expect(sandbox).toBeInstanceOf(SandboxError);
    expect(sandbox).not.toBeInstanceOf(DockerBuildError);
    expect(sandbox).not.toBeInstanceOf(ConfigurationError);

    expect(docker).toBeInstanceOf(DockerBuildError);
    expect(docker).not.toBeInstanceOf(SandboxError);

    expect(config).toBeInstanceOf(ConfigurationError);
    expect(config).not.toBeInstanceOf(SandboxError);
  });

  it('should all be instances of Error base class', () => {
    const errors = [
      new SandboxError('s', SandboxErrorCode.EXECUTION_TIMEOUT),
      new DockerBuildError('d', DockerErrorCode.BUILD_FAILED),
      new ConfigurationError('c', ConfigErrorCode.INVALID_SCHEMA),
    ];

    for (const error of errors) {
      expect(error).toBeInstanceOf(Error);
      expect(error.stack).toBeDefined();
    }
  });
});
