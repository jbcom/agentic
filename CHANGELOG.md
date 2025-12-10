# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-01-XX

### 🚀 Added

#### Sandbox Execution Module
- **NEW**: Complete Docker-based sandbox execution for AI agents
- **NEW**: `agentic sandbox run` command for single agent execution
- **NEW**: `agentic sandbox fleet` command for parallel agent execution
- **NEW**: Claude and Cursor runtime adapters
- **NEW**: Resource limits (memory, timeout) and workspace mounting
- **NEW**: Parallel execution with container isolation

#### Security & Quality
- **NEW**: Token sanitization in all error messages and logs
- **NEW**: Typed error classes with specific error codes (`SandboxError`, `DockerBuildError`, `ConfigurationError`)
- **NEW**: Safe subprocess execution utilities
- **NEW**: Property-based testing with fast-check (15 comprehensive properties)

#### Docker & CI/CD
- **NEW**: Multi-stage Dockerfile with Node.js 22 + Python 3.13
- **NEW**: Multi-architecture Docker builds (linux/amd64, linux/arm64)
- **NEW**: Automated Docker Hub publishing in CI
- **NEW**: Non-root execution with UID 1000 for security

#### Documentation
- **NEW**: Comprehensive installation guide (npm, Docker, development)
- **NEW**: Quickstart tutorial with working examples
- **NEW**: GitHub Pages documentation deployment
- **NEW**: Contributing guidelines and issue templates
- **NEW**: Pull request template

### 🔧 Enhanced

#### Core Features
- **Enhanced**: TypeScript strict mode compliance with complete type coverage
- **Enhanced**: Language separation - TypeScript-only repository
- **Enhanced**: CI/CD pipeline with comprehensive testing
- **Enhanced**: Package exports with sandbox module

#### Testing
- **Enhanced**: 47 total tests including 15 property-based tests
- **Enhanced**: Build output purity validation
- **Enhanced**: Package content purity validation
- **Enhanced**: API documentation completeness validation

### 🏗️ Technical

#### Architecture
- **Improved**: Clean module separation (core, fleet, triage, sandbox, github, handoff)
- **Improved**: Comprehensive TypeScript types and JSDoc comments
- **Improved**: Error handling with sanitization and typed errors

#### Dependencies
- **Added**: `fast-check` for property-based testing
- **Updated**: All dependencies to latest stable versions

### 📚 Documentation

- **Added**: Production-ready README with compelling examples
- **Added**: Real-world use cases and programmatic usage examples
- **Added**: Complete API reference documentation
- **Added**: Security best practices documentation

## [1.0.0] - 2024-XX-XX

### 🎉 Initial Release

#### Core Features
- **Fleet Management**: Spawn, monitor, and coordinate Cursor Background Agents
- **AI-Powered Triage**: Multi-provider AI analysis (Anthropic, OpenAI, Google, Mistral)
- **Intelligent Token Management**: Automatic organization-based token switching
- **Station-to-Station Handoff**: Seamless agent continuity across sessions
- **Multi-Org Support**: Manage agents across multiple GitHub organizations

#### CLI Interface
- `agentic fleet` - Fleet management commands
- `agentic triage` - AI analysis commands  
- `agentic tokens` - Token management commands
- `agentic handoff` - Agent handoff commands
- `agentic config` - Configuration management
- `agentic init` - Interactive setup

#### Security
- No hardcoded credentials or values
- User-configurable token management
- Safe subprocess execution
- ReDoS protection in regex patterns

#### Architecture
- TypeScript with strict mode
- ES Modules with NodeNext resolution
- Pluggable AI provider system
- Cosmiconfig-based configuration
- Comprehensive error handling

---

## Release Notes

### v1.1.0 - Production Ready 🚀

This release marks **agentic-control** as production-ready with major new capabilities:

**🏗️ Sandbox Execution**: Run AI agents in isolated Docker containers with resource limits and parallel execution support.

**🔒 Enterprise Security**: Token sanitization, typed errors, and safe subprocess execution make this ready for production environments.

**🧪 Quality Assurance**: 47 tests including 15 property-based tests ensure system reliability and correctness.

**📦 Easy Deployment**: Multi-architecture Docker images and comprehensive documentation make deployment simple.

**🎯 Real-World Ready**: From automated code maintenance to security auditing, this release handles complex enterprise workflows.

### Migration Guide

No breaking changes from v1.0.x. New sandbox functionality is additive.

### Upgrade Instructions

```bash
# npm/pnpm users
pnpm update -g agentic-control

# Docker users  
docker pull jbcom/agentic-control:latest

# Verify new features
agentic sandbox --help
```