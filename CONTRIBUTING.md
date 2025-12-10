# Contributing to agentic-control

Thank you for your interest in contributing to agentic-control! This guide will help you get started with contributing to this AI agent orchestration toolkit.

## 🚀 Quick Start

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/agentic-control.git
   cd agentic-control
   ```

2. **Install Dependencies**
   ```bash
   pnpm install
   ```

3. **Run Tests**
   ```bash
   pnpm test
   ```

4. **Start Development**
   ```bash
   pnpm run dev  # Watch mode
   pnpm run agentic --help  # Test CLI
   ```

## 📋 Development Guidelines

### Code Style

- **TypeScript**: Strict mode enabled, no implicit `any`
- **Formatting**: Prettier (run `pnpm run format`)
- **Linting**: ESLint (run `pnpm run lint`)
- **Imports**: Use `.js` extensions for ES modules

### Testing

We use **property-based testing** with fast-check for critical system properties:

```typescript
// Example property test
it('should sanitize tokens from error messages', () => {
  fc.assert(fc.property(
    fc.string().filter(s => s.includes('ghp_')),
    (input) => {
      const sanitized = sanitizeError(input);
      expect(sanitized).not.toContain('ghp_');
      expect(sanitized).toContain('[REDACTED_TOKEN]');
    }
  ));
});
```

**Test Categories:**
- **Unit tests**: Specific functionality
- **Property tests**: Universal properties across inputs
- **Integration tests**: End-to-end workflows

### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(sandbox): add parallel execution support
fix(tokens): handle edge case in org extraction
docs(readme): update installation instructions
test(fleet): add property test for agent spawning
```

**Types:**
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `test`: Test additions/changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

## 🏗️ Architecture Overview

```
src/
├── core/           # Shared utilities
│   ├── types.ts    # Type definitions
│   ├── tokens.ts   # Multi-org token management
│   ├── config.ts   # Configuration loading
│   ├── security.ts # Token sanitization
│   └── errors.ts   # Typed error classes
├── fleet/          # Agent fleet management
├── triage/         # AI-powered analysis
├── sandbox/        # Docker-based execution (NEW!)
├── github/         # GitHub API integration
├── handoff/        # Agent continuity
└── cli.ts          # Command-line interface
```

### Key Design Principles

1. **Security First**: No hardcoded credentials, token sanitization
2. **Type Safety**: Strict TypeScript, comprehensive types
3. **Testability**: Property-based testing for critical paths
4. **Modularity**: Clean separation of concerns
5. **User Experience**: Intelligent defaults, clear error messages

## 🔧 Adding New Features

### 1. Core Features

For core functionality (tokens, config, types):

```typescript
// src/core/new-feature.ts
export function newFeature(input: string): Result {
  // Implementation with proper error handling
  try {
    return { success: true, data: processInput(input) };
  } catch (error) {
    return { success: false, error: sanitizeError(error.message) };
  }
}
```

### 2. CLI Commands

Add new commands to `src/cli.ts`:

```typescript
const newCmd = program.command('new-feature')
  .description('Description of new feature')
  .argument('<input>', 'Input description')
  .option('--flag', 'Optional flag')
  .action(async (input, opts) => {
    try {
      const result = await newFeature(input, opts);
      if (opts.json) {
        output(result, true);
      } else {
        console.log(`✅ Success: ${result.message}`);
      }
    } catch (err) {
      console.error('❌ Failed:', err instanceof Error ? err.message : err);
      process.exit(1);
    }
  });
```

### 3. Property Tests

Add property tests for new features:

```typescript
describe('Property: New feature correctness', () => {
  it('should maintain invariant across all inputs', () => {
    fc.assert(fc.property(
      fc.string(),
      (input) => {
        const result = newFeature(input);
        // Test universal property
        expect(result).toSatisfy(someInvariant);
      }
    ));
  });
});
```

## 🧪 Testing Guidelines

### Running Tests

```bash
# All tests
pnpm test

# Watch mode
pnpm test --watch

# Coverage
pnpm run test:coverage

# Specific test file
pnpm test tests/tokens.test.ts
```

### Writing Tests

1. **Property tests** for universal behaviors
2. **Unit tests** for specific examples
3. **Integration tests** for end-to-end flows

### Test Structure

```typescript
describe('Feature Name', () => {
  describe('Property: Universal behavior', () => {
    it('should maintain property across all inputs', () => {
      // Property-based test with fast-check
    });
  });

  describe('Unit: Specific behaviors', () => {
    it('should handle specific case correctly', () => {
      // Traditional unit test
    });
  });
});
```

## 📚 Documentation

### Code Documentation

- **JSDoc comments** for all public APIs
- **Type annotations** for complex types
- **README updates** for new features

### Examples

Include working examples in documentation:

```typescript
/**
 * Spawns a new AI agent for the specified repository
 * 
 * @example
 * ```typescript
 * const fleet = new Fleet();
 * const result = await fleet.spawn({
 *   repository: 'https://github.com/my-org/my-repo',
 *   task: 'Fix the CI workflow',
 *   target: { autoCreatePr: true }
 * });
 * ```
 */
```

## 🔒 Security Considerations

### Token Handling

- **Never log tokens**: Use `sanitizeError()` for all error messages
- **Environment variables**: Store tokens in env vars, never in code
- **Validation**: Validate all inputs to prevent injection

### Subprocess Execution

- **Array arguments**: Use `spawn(['cmd', 'arg1', 'arg2'])` not `exec('cmd arg1 arg2')`
- **Input validation**: Validate all user inputs
- **Timeouts**: Set reasonable timeouts for all operations

### Error Handling

```typescript
import { sanitizeError, createSafeError } from './core/security.js';

try {
  // Operation that might expose tokens
} catch (error) {
  throw createSafeError('Operation failed', error);
}
```

## 🚀 Release Process

### Version Bumping

Versions are automatically bumped based on conventional commits:

- `feat:` → Minor version bump
- `fix:` → Patch version bump  
- `feat!:` or `BREAKING CHANGE:` → Major version bump

### Release Checklist

1. ✅ All tests passing
2. ✅ Documentation updated
3. ✅ CHANGELOG.md updated
4. ✅ Version bumped in package.json
5. ✅ Git tag created
6. ✅ npm package published
7. ✅ Docker image published
8. ✅ GitHub release created

## 🤝 Pull Request Process

1. **Create Feature Branch**
   ```bash
   git checkout -b feat/new-feature
   ```

2. **Make Changes**
   - Follow coding standards
   - Add tests for new functionality
   - Update documentation

3. **Test Locally**
   ```bash
   pnpm test
   pnpm run lint
   pnpm run typecheck
   ```

4. **Commit Changes**
   ```bash
   git commit -m "feat(scope): add new feature"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feat/new-feature
   ```

### PR Requirements

- ✅ All CI checks passing
- ✅ Tests added for new functionality
- ✅ Documentation updated
- ✅ Conventional commit messages
- ✅ No breaking changes (unless major version)

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment**: OS, Node.js version, package version
2. **Steps to reproduce**: Minimal example
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Error messages**: Full error output (tokens redacted)

## 💡 Feature Requests

For new features, please provide:

1. **Use case**: Why is this needed?
2. **Proposed solution**: How should it work?
3. **Alternatives**: Other approaches considered
4. **Breaking changes**: Any compatibility concerns

## 📞 Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and community support
- **Documentation**: [Full documentation](https://jbcom.github.io/agentic-control/)

## 🙏 Recognition

Contributors are recognized in:

- GitHub contributors list
- Release notes for significant contributions
- Special thanks in documentation

Thank you for contributing to agentic-control! 🚀