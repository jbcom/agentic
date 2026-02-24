use serde::{Deserialize, Serialize};

/// Simple conversation state for the wizard
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WizardConversationState {
    pub messages: Vec<SimpleMessage>,
    pub start_time: String,
    pub current_phase: String,
}

impl WizardConversationState {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            start_time: chrono::Utc::now().to_rfc3339(),
            current_phase: "design".to_string(),
        }
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.messages.push(SimpleMessage {
            role: "user".to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
    }

    pub fn add_assistant_message(&mut self, content: &str) {
        self.messages.push(SimpleMessage {
            role: "assistant".to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
    }

    pub fn get_history(&self) -> &[SimpleMessage] {
        &self.messages
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("# Game Design Conversation\n\n");
        markdown.push_str(&format!("Started: {}\n\n", self.start_time));

        for msg in &self.messages {
            let header = match msg.role.as_str() {
                "user" => "## User",
                "assistant" => "## AI Assistant",
                "system" => "## System",
                _ => "## Unknown",
            };
            markdown.push_str(&format!("{}\n{}\n\n", header, msg.content));
        }

        markdown
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.messages).unwrap_or_default()
    }
}

/// Simple message type without external dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleMessage {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

impl SimpleMessage {
    pub fn new(role: &str, content: String) -> Self {
        Self {
            role: role.to_string(),
            content,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn user(content: &str) -> Self {
        Self::new("user", content.to_string())
    }

    pub fn assistant(content: &str) -> Self {
        Self::new("assistant", content.to_string())
    }

    pub fn system(content: &str) -> Self {
        Self::new("system", content.to_string())
    }
}
