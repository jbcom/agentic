# Implementation Plan

- [x] 1. Complete Language Separation
  - Merge PR #9 to establish TypeScript-only repository
  - Verify CrewTool integration works with external agentic-crew CLI
  - Update package.json to remove Python dependencies
  - _Requirements: 1.1, 1.4_

- [x] 1.1 Write property test for build output purity
  - **Property 1: Build output purity**
  - **Validates: Requirements 1.2**

- [x] 1.2 Write property test for package content purity
  - **Property 2: Package content purity**
  - **Validates: Requirements 1.3**

- [x] 1.3 Write property test for CrewTool subprocess invocation
  - **Property 3: CrewTool subprocess invocation**
  - **Validates: Requirements 1.4**

- [x] 1.4 Write property test for type availability
  - **Property 4: Type availability**
  - **Validates: Requirements 1.5**

- [x] 2. Implement Docker Publishing Infrastructure
  - Create optimized Dockerfile with multi-stage builds
  - Set up Docker Hub authentication secrets in repository
  - Add Docker publishing workflow to .github/workflows/ci.yml
  - Configure multi-architecture builds (linux/amd64, linux/arm64)
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7_

- [x] 2.1 Write property test for Docker runtime versions
  - **Property 5: Docker runtime versions**
  - **Validates: Requirements 2.2**

- [x] 2.2 Write property test for Docker package installation
  - **Property 6: Docker package installation**
  - **Validates: Requirements 2.3**

- [x] 2.3 Write property test for multi-architecture support
  - **Property 7: Multi-architecture support**
  - **Validates: Requirements 2.5**

- [x] 2.4 Write example test for Docker non-root user
  - Verify Dockerfile uses UID 1000 for agent user
  - **Validates: Requirements 2.6, 7.4**

- [x] 3. Implement Sandbox Execution Module
- [x] 3.1 Create sandbox module structure and interfaces
  - Create src/sandbox/ directory with index.ts, container.ts, runtime/, workspace.ts, output.ts, config.ts
  - Define ContainerManager, RuntimeAdapter, and SandboxExecutor interfaces
  - Implement base RuntimeAdapter abstract class
  - _Requirements: 3.1_

- [x] 3.2 Implement ContainerManager for Docker lifecycle
  - Write Docker container creation, start, stop, remove operations
  - Add container logging and status monitoring
  - Implement resource limit enforcement (memory, timeout)
  - _Requirements: 3.2, 3.3_

- [x] 3.3 Write property test for workspace mounting
  - **Property 8: Workspace mounting**
  - **Validates: Requirements 3.2**

- [x] 3.4 Write property test for resource limit enforcement
  - **Property 9: Resource limit enforcement**
  - **Validates: Requirements 3.3**

- [x] 3.5 Implement Claude runtime adapter
  - Create ClaudeRuntime class implementing RuntimeAdapter
  - Add command preparation for Claude Agent SDK
  - Implement output parsing for Claude responses
  - Add environment validation for ANTHROPIC_API_KEY
  - _Requirements: 3.5_

- [x] 3.6 Implement Cursor runtime adapter
  - Create CursorRuntime class implementing RuntimeAdapter
  - Add command preparation for Cursor agents
  - Implement output parsing for Cursor responses
  - Add environment validation for CURSOR_API_KEY
  - _Requirements: 3.5_

- [x] 3.7 Write property test for output extraction
  - **Property 10: Output extraction**
  - **Validates: Requirements 3.4**

- [x] 3.8 Write property test for runtime adapter selection
  - **Property 11: Runtime adapter selection**
  - **Validates: Requirements 3.5**

- [x] 3.9 Implement SandboxExecutor with parallel execution support
  - Create SandboxExecutor class with execute() and executeFleet() methods
  - Add parallel container management without conflicts
  - Implement proper cleanup and error handling
  - _Requirements: 3.6_

- [x] 3.10 Write property test for parallel sandbox isolation
  - **Property 12: Parallel sandbox isolation**
  - **Validates: Requirements 3.6**

- [x] 3.11 Add sandbox CLI commands
  - Implement 'agentic sandbox run' command with runtime, workspace, output options
  - Add 'agentic sandbox fleet' command for parallel execution
  - Add timeout, memory, and environment variable support
  - _Requirements: 3.5_

- [x] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [-] 5. Set Up Comprehensive Documentation System
- [x] 5.1 Configure Sphinx with TypeScript and Python extensions
  - Update docs/conf.py with sphinx-js, myst-parser, sphinxcontrib-mermaid
  - Configure TypeScript API documentation generation
  - Set up cross-references between TypeScript and Python docs
  - _Requirements: 4.1, 4.2_

- [x] 5.2 Write property test for API documentation completeness
  - **Property 13: API documentation completeness**
  - **Validates: Requirements 4.1**

- [x] 5.3 Create installation and quickstart guides
  - Write docs/getting-started/installation.md with both npm and Docker instructions
  - Create docs/getting-started/quickstart.md with working examples
  - Add configuration examples for multi-org setups
  - _Requirements: 4.3, 4.4, 6.1, 6.3, 6.4, 6.5_

- [x] 5.4 Write example tests for documentation files
  - Verify installation guide exists and contains required sections
  - Verify quickstart tutorial exists with code examples
  - Verify architecture documentation exists
  - **Validates: Requirements 4.3, 4.4, 4.5**

- [x] 5.5 Create comprehensive usage guides
  - Write docs/guides/fleet-management.md
  - Write docs/guides/triage.md
  - Write docs/guides/sandbox.md (new)
  - Write docs/guides/configuration.md
  - _Requirements: 6.4_

- [x] 5.6 Set up GitHub Pages deployment
  - Add docs workflow to .github/workflows/docs.yml
  - Configure automatic deployment on main branch updates
  - Test documentation build and deployment
  - _Requirements: 4.6_

- [x] 6. Enhance Configuration and Examples
- [x] 6.1 Improve agentic init command
  - Enhance configuration detection and generation
  - Add validation for generated configurations
  - Provide better error messages for missing environment variables
  - _Requirements: 6.2, 7.5_

- [x] 6.2 Write property test for configuration generation validity
  - **Property 14: Configuration generation validity**
  - **Validates: Requirements 6.2**

- [x] 6.3 Write property test for environment variable error messages
  - **Property 18: Environment variable error messages**
  - **Validates: Requirements 7.5**

- [x] 6.4 Create example configuration files
  - Add docs/examples/basic-usage.md with simple configurations
  - Add docs/examples/advanced-patterns.md with multi-org setups
  - Add docs/examples/integration.md with programmatic usage
  - _Requirements: 6.1, 6.3_

- [x] 6.5 Write example test for repository structure
  - Verify example configuration files exist in docs directory
  - **Validates: Requirements 6.1**

- [x] 7. Implement Security and Quality Measures
- [x] 7.1 Implement token sanitization
  - Add token sanitization to all error messages and logs
  - Create utility functions for safe error reporting
  - Update existing error handling to use sanitization
  - _Requirements: 7.1_

- [x] 7.2 Write property test for token sanitization
  - **Property 15: Token sanitization**
  - **Validates: Requirements 7.1**

- [x] 7.3 Audit subprocess execution for safety
  - Review all subprocess calls to ensure array-based arguments
  - Replace any shell interpolation with safe execution methods
  - Add validation for subprocess inputs
  - _Requirements: 7.2_

- [x] 7.4 Write property test for safe subprocess execution
  - **Property 16: Safe subprocess execution**
  - **Validates: Requirements 7.2**

- [x] 7.5 Implement comprehensive configuration validation
  - Add Zod schemas for all configuration types
  - Implement validation with clear error messages
  - Add configuration testing utilities
  - _Requirements: 7.3_

- [x] 7.6 Write property test for configuration validation
  - **Property 17: Configuration validation**
  - **Validates: Requirements 7.3**

- [ ] 8. Set Up Contribution Guidelines and Templates
- [x] 8.1 Create contribution documentation
  - Write CONTRIBUTING.md with development setup, coding standards, PR process
  - Add issue templates for bugs and feature requests
  - Create PR template with checklist
  - _Requirements: 8.1, 8.5_

- [x] 8.2 Write example tests for contribution files
  - Verify CONTRIBUTING.md exists
  - Verify issue templates exist in .github/ISSUE_TEMPLATE/
  - **Validates: Requirements 8.1, 8.5**

- [ ] 8.3 Implement documentation example validation
  - Create test utilities to extract and validate code examples
  - Add tests to verify all documentation examples are syntactically correct
  - Set up automated testing of example code
  - _Requirements: 8.4_

- [ ] 8.4 Write property test for documentation example validity
  - **Property 19: Documentation example validity**
  - **Validates: Requirements 8.4**

- [ ] 9. Implement Versioning and Release Management
- [ ] 9.1 Enhance release automation
  - Improve version detection and bumping logic in CI
  - Add semantic versioning validation
  - Ensure consistency between package.json and git tags
  - _Requirements: 9.1, 9.2_

- [ ] 9.2 Write property test for semantic versioning compliance
  - **Property 20: Semantic versioning compliance**
  - **Validates: Requirements 9.1**

- [ ] 9.3 Write property test for version consistency
  - **Property 21: Version consistency**
  - **Validates: Requirements 9.2**

- [ ] 9.4 Improve release notes generation
  - Enhance GitHub release creation with better formatting
  - Add changelog generation from conventional commits
  - Include Docker image information in releases
  - _Requirements: 9.3_

- [x] 10. Enhance TypeScript Type Safety and Documentation
- [x] 10.1 Audit and improve TypeScript types
  - Review all exports for complete type coverage
  - Add JSDoc comments to all public interfaces
  - Ensure strict TypeScript mode compliance
  - _Requirements: 10.1, 10.2, 10.3, 10.4_

- [x] 10.2 Write example test for TypeScript strict mode
  - Verify tsconfig.json has strict mode enabled
  - Verify no implicit any types in configuration
  - **Validates: Requirements 10.2**

- [x] 10.3 Write property test for declaration file completeness
  - **Property 22: Declaration file completeness**
  - **Validates: Requirements 10.1**

- [x] 10.4 Write property test for JSDoc completeness
  - **Property 23: JSDoc completeness**
  - **Validates: Requirements 10.3**

- [x] 10.5 Write property test for type inference quality
  - **Property 24: Type inference quality**
  - **Validates: Requirements 10.4**

- [x] 10.6 Implement typed error classes
  - Create SandboxError, DockerBuildError, ConfigurationError classes
  - Add specific error codes for different failure types
  - Update all error throwing to use typed errors
  - _Requirements: 10.5_

- [x] 10.7 Write property test for typed error classes
  - **Property 25: Typed error classes**
  - **Validates: Requirements 10.5**

- [x] 11. Final Testing and Quality Assurance
- [x] 11.1 Implement comprehensive test suite
  - Set up property-based testing with fast-check
  - Configure test coverage reporting with vitest
  - Add integration tests for Docker and sandbox functionality
  - _Requirements: 5.1, 5.2_

- [x] 11.2 Test Docker image end-to-end
  - Build Docker image locally and verify functionality
  - Test both agentic-control and agentic-crew commands work
  - Verify multi-architecture builds work correctly
  - _Requirements: 2.2, 2.3, 2.5_

- [x] 11.3 Test CI/CD workflows
  - Verify all CI jobs pass on main branch
  - Test release automation with staging environment
  - Verify Docker publishing works correctly
  - _Requirements: 5.3, 5.4, 5.5, 5.6_

- [x] 12. Final Checkpoint - Production Readiness
  - Ensure all tests pass, ask the user if questions arise.
  - Verify documentation builds and deploys correctly
  - Confirm Docker images publish successfully
  - Validate all examples work as documented
  - Review security measures and error handling
  - Prepare v1.0 release notes and announcement