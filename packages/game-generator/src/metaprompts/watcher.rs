use anyhow::{Context, Result};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

pub struct PromptWatcher {
    _config_watcher: Option<RecommendedWatcher>,
    _prompts_watcher: Option<RecommendedWatcher>,
    _build_watcher: Option<RecommendedWatcher>,
    config_rx: Receiver<notify::Result<Event>>,
    prompts_rx: Receiver<notify::Result<Event>>,
    build_rx: Receiver<notify::Result<Event>>,
    generation_queue: GenerationQueue,
}

impl PromptWatcher {
    pub fn new(config_dir: PathBuf, prompts_dir: PathBuf, build_dir: PathBuf) -> Result<Self> {
        let (config_tx, config_rx) = channel();
        let (prompts_tx, prompts_rx) = channel();
        let (build_tx, build_rx) = channel();

        // Create config watcher
        let mut config_watcher = RecommendedWatcher::new(config_tx, Config::default())?;
        config_watcher.watch(&config_dir, RecursiveMode::NonRecursive)?;

        // Create prompts watcher
        let mut prompts_watcher = RecommendedWatcher::new(prompts_tx, Config::default())?;
        prompts_watcher.watch(&prompts_dir, RecursiveMode::Recursive)?;

        // Create build watcher
        let mut build_watcher = RecommendedWatcher::new(build_tx, Config::default())?;
        build_watcher.watch(&build_dir, RecursiveMode::Recursive)?;

        Ok(Self {
            _config_watcher: Some(config_watcher),
            _prompts_watcher: Some(prompts_watcher),
            _build_watcher: Some(build_watcher),
            config_rx,
            prompts_rx,
            build_rx,
            generation_queue: GenerationQueue::new(),
        })
    }

    pub fn check_for_changes(&mut self) -> Vec<FileChangeEvent> {
        let mut events = Vec::new();

        // Check config changes
        while let Ok(Ok(event)) = self.config_rx.try_recv() {
            if let Some(change) = self.process_event(event, WatcherType::Config) {
                events.push(change);
            }
        }

        // Check prompt changes
        while let Ok(Ok(event)) = self.prompts_rx.try_recv() {
            if let Some(change) = self.process_event(event, WatcherType::Prompts) {
                events.push(change);
            }
        }

        // Check build changes
        while let Ok(Ok(event)) = self.build_rx.try_recv() {
            if let Some(change) = self.process_event(event, WatcherType::Build) {
                events.push(change);
            }
        }

        events
    }

    fn process_event(
        &mut self,
        event: Event,
        watcher_type: WatcherType,
    ) -> Option<FileChangeEvent> {
        match event.kind {
            notify::EventKind::Create(_)
            | notify::EventKind::Modify(_)
            | notify::EventKind::Remove(_) => {
                if let Some(path) = event.paths.first() {
                    // Filter out temporary files and non-relevant extensions
                    if self.is_relevant_file(path, &watcher_type) {
                        return Some(FileChangeEvent {
                            path: path.clone(),
                            event_type: event.kind,
                            watcher_type,
                            timestamp: Instant::now(),
                        });
                    }
                }
            }
            _ => {}
        }
        None
    }

    fn is_relevant_file(&self, path: &Path, watcher_type: &WatcherType) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            match watcher_type {
                WatcherType::Config => ext_str == "toml" || ext_str == "yaml",
                WatcherType::Prompts => ext_str == "jinja" || ext_str == "jinja2",
                WatcherType::Build => ext_str == "rs" || ext_str == "toml" || ext_str == "png",
            }
        } else {
            false
        }
    }

    pub fn queue_generation(&mut self, task: GenerationTask) {
        self.generation_queue.push(task);
    }

    pub fn get_next_task(&mut self) -> Option<GenerationTask> {
        self.generation_queue.pop()
    }
}

#[derive(Clone, Debug)]
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub event_type: notify::EventKind,
    pub watcher_type: WatcherType,
    pub timestamp: Instant,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WatcherType {
    Config,
    Prompts,
    Build,
}

pub struct GenerationQueue {
    tasks: VecDeque<GenerationTask>,
    rate_limiter: RateLimiter,
}

impl Default for GenerationQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl GenerationQueue {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            rate_limiter: RateLimiter::new(Duration::from_secs(2)), // 2 second minimum between tasks
        }
    }

    pub fn push(&mut self, task: GenerationTask) {
        // Check if we already have a similar task queued
        if !self.tasks.iter().any(|t| t.is_similar(&task)) {
            self.tasks.push_back(task);
        }
    }

    pub fn pop(&mut self) -> Option<GenerationTask> {
        if self.rate_limiter.can_proceed() {
            self.tasks.pop_front()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct GenerationTask {
    pub task_type: GenerationTaskType,
    pub priority: u8,
    pub created_at: Instant,
    pub metadata: serde_json::Value,
}

impl GenerationTask {
    pub fn new(task_type: GenerationTaskType, priority: u8) -> Self {
        Self {
            task_type,
            priority,
            created_at: Instant::now(),
            metadata: serde_json::Value::Null,
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    fn is_similar(&self, other: &GenerationTask) -> bool {
        match (&self.task_type, &other.task_type) {
            (GenerationTaskType::ValidatePrompt(a), GenerationTaskType::ValidatePrompt(b)) => {
                a == b
            }
            (GenerationTaskType::GenerateAssets(a), GenerationTaskType::GenerateAssets(b)) => {
                a == b
            }
            (GenerationTaskType::UpdateCode(a), GenerationTaskType::UpdateCode(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenerationTaskType {
    ValidatePrompt(PathBuf),
    GenerateAssets(String), // Asset type
    UpdateCode(PathBuf),
    RegenerateWorld,
    RefreshStyleGuide,
}

struct RateLimiter {
    last_action: Option<Instant>,
    min_interval: Duration,
}

impl RateLimiter {
    fn new(min_interval: Duration) -> Self {
        Self {
            last_action: None,
            min_interval,
        }
    }

    fn can_proceed(&mut self) -> bool {
        if let Some(last) = self.last_action
            && last.elapsed() < self.min_interval
        {
            return false;
        }
        self.last_action = Some(Instant::now());
        true
    }
}

// Helper function from the user's example
pub fn check_api_key() -> Result<String> {
    std::env::var("OPENAI_API_KEY").context(
        "OPENAI_API_KEY environment variable not set.\n\
                  Please set it with: export OPENAI_API_KEY='your-key-here'",
    )
}
