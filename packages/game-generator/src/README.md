# Source Code Structure

This directory contains the main implementation of the AI RPG Generator Studio. The code is organized into distinct modules following a clean architecture pattern.

## Module Overview

### `lib.rs`
The library entry point that exports the main `GameGeneratorStudioPlugin` and defines the plugin structure. It initializes the tokio runtime and registers all sub-plugins.

Key exports:
- `GameGeneratorStudioPlugin` - Main Bevy plugin
- `TokioRuntime` - Shared async runtime resource
- `StudioDatabase` - Database connection manager
- `StudioServices` - Service layer facade
- `BevyGameGenerator` - AI generation engine
- `GameConcept` - Core game configuration model

### `main.rs`
The executable entry point that creates the Bevy application and initializes the database connection before starting the studio. Handles:
- Database initialization with default location
- Service layer setup
- Bevy app configuration
- Plugin registration

## Core Modules

### `database/`
Database layer using SeaORM for persistence. Handles:
- Project management and state persistence
- Asset storage with versioning
- Generation task queuing
- API key encryption and storage
- Wizard state tracking

### `generator/`
The AI-powered game generation engine featuring:
- Metaprompt orchestration (5-phase system)
- Multi-modal generation (text, images, audio)
- Style consistency enforcement
- Token management with tiktoken-rs
- Asset validation and retry logic

### `services/`
Business logic layer providing clean APIs for:
- Project CRUD operations
- Asset management and versioning
- Generation task orchestration
- Wizard state management
- Cross-cutting concerns

### `studio/`
Main IDE interface built with egui featuring:
- Dockable window system
- Live game preview
- Asset gallery
- Code editor integration
- Export functionality

### `wizard/`
Project configuration wizard providing:
- Step-by-step game setup
- Visual configuration UI
- Validation at each step
- Progress persistence
- Template selection

## Architecture Principles

1. **Plugin-Based**: Each major system is a Bevy plugin
2. **Async Integration**: Tokio runtime for non-blocking operations
3. **Clean Architecture**: Clear separation of concerns
4. **Event-Driven**: Communication via Bevy events
5. **Database-Backed**: All state persists to SQLite

## Key Patterns

### Resource Management
```rust
// Shared runtime for async operations
app.insert_resource(TokioRuntime(Arc::new(runtime)));

// Service layer access
app.insert_resource(StudioServices::new(db));
```

### Event Communication
```rust
// Database operations via events
#[derive(Event)]
pub enum DatabaseTaskEvent {
    CreateAsset { name: String, data: Vec<u8> },
    QueueGeneration { task_type: String, prompt: String },
}
```

### State Management
```rust
// Bevy states for app phases
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum StudioPhase {
    #[default]
    Setup,
    Generation,
    Preview,
    Export,
}
