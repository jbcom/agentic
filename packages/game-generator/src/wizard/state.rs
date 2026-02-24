use crate::metaprompts::GenerationPhase;
use crate::wizard::config::ConfigManager;
use crate::wizard::steps::guided::GuidedModeExport;
use crate::wizard::steps::{LanguageChoice, WelcomeAction};
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    Welcome,
    SelectLanguage,
    GuidedMode,   // Handled by guided.rs
    FreeformMode, // Handled by freeform.rs
    Review,
    Complete, // After successful export
}

#[derive(Debug, Clone, PartialEq)]
pub enum WizardMode {
    NotSelected,
    Guided,
    Freeform,
}

#[derive(Resource)]
pub struct AppState {
    pub wizard_step: WizardStep,
    pub wizard_mode: WizardMode,
    pub selected_language: Option<LanguageChoice>,
    pub form_data: HashMap<String, String>,
    pub generation_status: GenerationStatus,
    pub error_message: Option<String>,
    pub show_exit_dialog: bool,
    pub guided_export: Option<GuidedModeExport>,

    // Generation pipeline fields
    pub generation_active: bool,
    pub prompt_validation_queue: Vec<PromptValidation>,
    pub current_phase: GenerationPhase,
    pub generation_logs: Vec<(LogLevel, String)>,

    // Configuration manager for persisting wizard state
    pub config_manager: Option<ConfigManager>,
}

#[derive(Debug, Clone)]
pub struct PromptValidation {
    pub path: PathBuf,
    pub phase: String,
    pub name: String,
    pub content: String,
    pub validated: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenerationStatus {
    Idle,
    Generating { current: String, progress: f32 },
    Complete,
    Failed(String),
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            wizard_step: WizardStep::Welcome,
            wizard_mode: WizardMode::NotSelected,
            selected_language: None,
            form_data: HashMap::new(),
            generation_status: GenerationStatus::Idle,
            error_message: None,
            show_exit_dialog: false,
            guided_export: None,
            generation_active: false,
            prompt_validation_queue: Vec::new(),
            current_phase: GenerationPhase::Design,
            generation_logs: Vec::new(),
            config_manager: None,
        }
    }

    pub fn set_config_manager(&mut self, config_manager: ConfigManager) {
        self.config_manager = Some(config_manager);
    }

    pub fn set_wizard_mode(&mut self, action: WelcomeAction) {
        match action {
            WelcomeAction::GuidedMode => {
                self.wizard_mode = WizardMode::Guided;
                self.wizard_step = WizardStep::SelectLanguage;
            }
            WelcomeAction::FreeformMode => {
                self.wizard_mode = WizardMode::Freeform;
                self.wizard_step = WizardStep::SelectLanguage;
            }
        }
    }

    pub fn set_language(&mut self, choice: LanguageChoice) {
        self.selected_language = Some(choice);

        // Move to next appropriate step based on mode
        match self.wizard_mode {
            WizardMode::Guided => {
                self.wizard_step = WizardStep::GuidedMode;
            }
            WizardMode::Freeform => {
                self.wizard_step = WizardStep::FreeformMode;
            }
            WizardMode::NotSelected => {
                // This shouldn't happen, but handle gracefully
                error!("Language selected without mode selection");
            }
        }
    }

    pub fn can_go_back(&self) -> bool {
        !matches!(&self.wizard_step, WizardStep::Welcome)
    }

    pub fn go_back(&mut self) {
        self.wizard_step = match &self.wizard_step {
            WizardStep::Welcome => WizardStep::Welcome,
            WizardStep::SelectLanguage => {
                self.wizard_mode = WizardMode::NotSelected;
                WizardStep::Welcome
            }
            WizardStep::GuidedMode | WizardStep::FreeformMode => {
                self.selected_language = None;
                WizardStep::SelectLanguage
            }
            WizardStep::Review => match self.wizard_mode {
                WizardMode::Guided => WizardStep::GuidedMode,
                WizardMode::Freeform => WizardStep::FreeformMode,
                WizardMode::NotSelected => WizardStep::Welcome,
            },
            WizardStep::Complete => {
                // Can't go back from complete - stay at complete
                WizardStep::Complete
            }
        };
    }

    pub fn get_progress(&self) -> f32 {
        match (&self.wizard_step, &self.wizard_mode) {
            (WizardStep::Welcome, _) => 0.1,
            (WizardStep::SelectLanguage, _) => 0.2,
            (WizardStep::GuidedMode, _) => 0.5,
            (WizardStep::FreeformMode, _) => 0.5,
            (WizardStep::Review, _) => 0.9,
            (WizardStep::Complete, _) => 1.0,
        }
    }

    pub fn get_step_title(&self) -> &str {
        match &self.wizard_step {
            WizardStep::Welcome => "Welcome",
            WizardStep::SelectLanguage => "Select Language",
            WizardStep::GuidedMode => "Browse & Blend Vintage Games",
            WizardStep::FreeformMode => "AI-Assisted Game Creation",
            WizardStep::Review => "Review & Generate",
            WizardStep::Complete => "Generation Complete",
        }
    }

    pub fn is_generating(&self) -> bool {
        matches!(self.generation_status, GenerationStatus::Generating { .. })
    }

    pub fn start_generation(&mut self, task: String) {
        self.generation_status = GenerationStatus::Generating {
            current: task,
            progress: 0.0,
        };
    }

    pub fn update_generation_progress(&mut self, task: String, progress: f32) {
        self.generation_status = GenerationStatus::Generating {
            current: task,
            progress,
        };
    }

    pub fn complete_generation(&mut self) {
        self.generation_status = GenerationStatus::Complete;
    }

    pub fn fail_generation(&mut self, error: String) {
        self.generation_status = GenerationStatus::Failed(error);
    }

    pub fn set_guided_export(&mut self, export: GuidedModeExport) {
        self.guided_export = Some(export);
    }

    pub fn set_wizard_step(&mut self, step: WizardStep) {
        self.wizard_step = step;
    }

    // Generation pipeline methods
    pub fn get_next_prompt_to_validate(&self) -> Option<&PromptValidation> {
        self.prompt_validation_queue
            .iter()
            .find(|p| !p.validated && p.errors.is_empty())
    }

    pub fn add_log(&mut self, level: LogLevel, message: String) {
        self.generation_logs.push((level, message));
    }

    pub fn mark_prompt_validated(&mut self, path: &PathBuf, errors: Vec<String>) {
        if let Some(prompt) = self
            .prompt_validation_queue
            .iter_mut()
            .find(|p| &p.path == path)
        {
            prompt.validated = true;
            prompt.errors = errors;
        }
    }

    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            GenerationPhase::Initializing => GenerationPhase::Design,
            GenerationPhase::Design => GenerationPhase::StyleGuide,
            GenerationPhase::DesigningCore => GenerationPhase::StyleGuide,
            GenerationPhase::StyleGuide => GenerationPhase::WorldGeneration,
            GenerationPhase::WorldGeneration => GenerationPhase::AiSystems,
            GenerationPhase::AiSystems => GenerationPhase::AssetGeneration,
            GenerationPhase::AssetGeneration => GenerationPhase::CodeGeneration,
            GenerationPhase::CodeGeneration => GenerationPhase::DialogWriting,
            GenerationPhase::DialogWriting => GenerationPhase::MusicComposition,
            GenerationPhase::MusicComposition => GenerationPhase::Integration,
            GenerationPhase::Integration => GenerationPhase::Testing,
            GenerationPhase::Testing => GenerationPhase::Packaging,
            GenerationPhase::Packaging
            | GenerationPhase::Finalizing
            | GenerationPhase::Complete => {
                GenerationPhase::Complete // Stay at final phase
            }
            // Legacy aliases map to their equivalents
            GenerationPhase::GameDesign => GenerationPhase::StyleGuide,
            GenerationPhase::SpriteGeneration
            | GenerationPhase::TilesetGeneration
            | GenerationPhase::GeneratingAssets => GenerationPhase::AssetGeneration,
            GenerationPhase::WritingDialogue => GenerationPhase::DialogWriting,
            GenerationPhase::ComposingMusic => GenerationPhase::MusicComposition,
        };
    }

    pub fn add_prompt_to_validate(
        &mut self,
        phase: String,
        name: String,
        content: String,
        path: PathBuf,
    ) {
        self.prompt_validation_queue.push(PromptValidation {
            path,
            phase,
            name,
            content,
            validated: false,
            errors: Vec::new(),
        });
    }

    pub fn set_game_specification<T>(&mut self, _config: T) {
        // TODO: Implement when we have proper game specification type
        self.add_log(LogLevel::Info, "Game specification updated".to_string());
    }
}

// Legacy WizardState for compatibility - will be removed later
#[derive(Debug, Clone)]
pub struct WizardState {
    pub current_step: usize,
    pub total_steps: usize,
    pub project_name: String,
    pub can_proceed: bool,
    pub form_data: HashMap<String, String>,
}

impl Default for WizardState {
    fn default() -> Self {
        Self {
            current_step: 0,
            total_steps: 6,
            project_name: String::new(),
            can_proceed: false,
            form_data: HashMap::new(),
        }
    }
}
