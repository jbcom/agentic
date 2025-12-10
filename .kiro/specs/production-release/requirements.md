# Requirements Document

## Introduction

This document defines the requirements for preparing **agentic-control** for production release to the open-source community. The project is a unified AI agent fleet management, triage, and orchestration toolkit with a TypeScript core (npm) and Python companion package (PyPI). Based on analysis of the current state, CI/CD workflows, open issues, and recent refactoring work, this spec addresses the remaining gaps before a stable v1.0 production release.

## Glossary

- **agentic-control**: The TypeScript package providing fleet management, triage, and orchestration capabilities
- **agentic-crew**: The Python companion package providing CrewAI-based autonomous development crews
- **Fleet**: A collection of managed AI agents (Cursor Background Agents)
- **Triage**: AI-powered analysis of conversations, code, and tasks
- **Sandbox**: Isolated execution environment for running AI agents locally
- **MCP**: Model Context Protocol for agent communication
- **CI/CD**: Continuous Integration and Continuous Deployment pipeline
- **npm**: Node Package Manager registry for TypeScript packages
- **PyPI**: Python Package Index registry for Python packages
- **Docker Hub**: Container registry for Docker images

## Requirements

### Requirement 1

**User Story:** As a developer, I want the TypeScript package to be cleanly separated from Python code, so that I can use the npm package without Python dependencies.

#### Acceptance Criteria

1. WHEN PR #9 is merged THEN the system SHALL contain only TypeScript code in the main repository
2. WHEN the package is built THEN the system SHALL produce only TypeScript artifacts in the dist directory
3. WHEN users install from npm THEN the system SHALL NOT include any Python files or dependencies
4. WHEN the CrewTool is invoked THEN the system SHALL call the external agentic-crew CLI via subprocess
5. WHEN the package exports are imported THEN the system SHALL provide TypeScript types for all crew-related operations

### Requirement 2

**User Story:** As a DevOps engineer, I want Docker images published to Docker Hub, so that I can deploy agentic-control in containerized environments.

#### Acceptance Criteria

1. WHEN code is pushed to main THEN the system SHALL build a multi-stage Docker image
2. WHEN the Docker image is built THEN the system SHALL include Node.js 22 and Python 3.13 runtimes
3. WHEN the Docker image is built THEN the system SHALL install both agentic-control and agentic-crew packages
4. WHEN the Docker image is published THEN the system SHALL push to Docker Hub with semantic version tags
5. WHEN the Docker image is published THEN the system SHALL support linux/amd64 and linux/arm64 architectures
6. WHEN the Dockerfile is created THEN the system SHALL use non-root execution with UID 1000
7. WHEN the Docker build completes THEN the system SHALL optimize image size using multi-stage builds

### Requirement 3

**User Story:** As a developer, I want sandbox execution capabilities, so that I can run AI agents in isolated local environments.

#### Acceptance Criteria

1. WHEN the sandbox module is implemented THEN the system SHALL provide a container lifecycle management interface
2. WHEN a sandbox is created THEN the system SHALL mount the specified workspace directory
3. WHEN a sandbox agent executes THEN the system SHALL enforce memory and timeout limits
4. WHEN a sandbox completes THEN the system SHALL extract output to the specified directory
5. WHEN the CLI is invoked with sandbox commands THEN the system SHALL support Claude and Cursor runtime adapters
6. WHEN multiple sandboxes run THEN the system SHALL manage parallel execution without conflicts

### Requirement 4

**User Story:** As a project maintainer, I want comprehensive documentation, so that users can understand and adopt the toolkit effectively.

#### Acceptance Criteria

1. WHEN documentation is generated THEN the system SHALL include API reference for all public TypeScript modules
2. WHEN documentation is generated THEN the system SHALL include API reference for all public Python modules
3. WHEN users visit the documentation site THEN the system SHALL provide installation guides for both packages
4. WHEN users visit the documentation site THEN the system SHALL provide quickstart tutorials with working examples
5. WHEN users visit the documentation site THEN the system SHALL document the architecture and integration patterns
6. WHEN documentation is built THEN the system SHALL publish to GitHub Pages automatically on main branch updates

### Requirement 5

**User Story:** As a developer, I want all tests passing and CI/CD working reliably, so that I can trust the stability of releases.

#### Acceptance Criteria

1. WHEN CI runs on main THEN the system SHALL pass all TypeScript build and test jobs
2. WHEN CI runs on main THEN the system SHALL pass all Python build, test, and lint jobs
3. WHEN changes are pushed to main THEN the system SHALL automatically release to npm if TypeScript code changed
4. WHEN changes are pushed to main THEN the system SHALL automatically release to PyPI if Python code changed
5. WHEN releases are created THEN the system SHALL generate GitHub release notes with version information
6. WHEN the release process fails THEN the system SHALL provide clear error messages and not create partial releases

### Requirement 6

**User Story:** As a user, I want clear examples and working configurations, so that I can get started quickly without trial and error.

#### Acceptance Criteria

1. WHEN the repository is cloned THEN the system SHALL include example configuration files in the docs directory
2. WHEN users run agentic init THEN the system SHALL generate a working configuration with detected values
3. WHEN example code is provided THEN the system SHALL include both CLI and programmatic usage patterns
4. WHEN examples are documented THEN the system SHALL cover all major use cases including fleet, triage, and handoff
5. WHEN configuration examples are shown THEN the system SHALL demonstrate multi-org token management

### Requirement 7

**User Story:** As a security-conscious user, I want the toolkit to follow security best practices, so that I can use it safely in production environments.

#### Acceptance Criteria

1. WHEN tokens are processed THEN the system SHALL never log or expose token values in error messages
2. WHEN subprocess commands are executed THEN the system SHALL use safe execution methods without shell interpolation
3. WHEN configuration is loaded THEN the system SHALL validate all inputs against defined schemas
4. WHEN Docker images are built THEN the system SHALL run as non-root user
5. WHEN environment variables are accessed THEN the system SHALL provide clear error messages for missing required values

### Requirement 8

**User Story:** As an open-source contributor, I want clear contribution guidelines, so that I can contribute effectively to the project.

#### Acceptance Criteria

1. WHEN the repository is viewed THEN the system SHALL include a CONTRIBUTING.md file with guidelines
2. WHEN contributors submit PRs THEN the system SHALL enforce conventional commit message format
3. WHEN code is submitted THEN the system SHALL require passing tests and linting checks
4. WHEN documentation is updated THEN the system SHALL validate that examples are accurate and working
5. WHEN issues are created THEN the system SHALL provide issue templates for bugs and features

### Requirement 9

**User Story:** As a package maintainer, I want proper versioning and changelog management, so that users can track changes between releases.

#### Acceptance Criteria

1. WHEN releases are created THEN the system SHALL follow semantic versioning for both npm and PyPI packages
2. WHEN version numbers are updated THEN the system SHALL maintain consistency between package.json and git tags
3. WHEN releases are published THEN the system SHALL generate release notes from commit messages
4. WHEN breaking changes are introduced THEN the system SHALL increment the major version number
5. WHEN new features are added THEN the system SHALL increment the minor version number

### Requirement 10

**User Story:** As a developer integrating agentic-control, I want comprehensive TypeScript types, so that I can use the toolkit with full IDE support and type safety.

#### Acceptance Criteria

1. WHEN the package is installed THEN the system SHALL provide TypeScript declaration files for all exports
2. WHEN types are defined THEN the system SHALL use strict TypeScript mode with no implicit any
3. WHEN interfaces are exported THEN the system SHALL document all properties with JSDoc comments
4. WHEN the package is used THEN the system SHALL provide type inference for all public APIs
5. WHEN errors occur THEN the system SHALL use typed error classes with specific error codes
