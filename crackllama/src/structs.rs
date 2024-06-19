use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    Llama38B,
    Llama370B,
}

impl Model {
    pub fn get_model_name(&self) -> String {
        match self {
            Model::Llama38B => "llama3-8b-8192".to_string(),
            Model::Llama370B => "llama3-70b-8192".to_string(),
        }
    }

    pub fn available_models() -> Vec<String> {
        vec![
            Model::Llama38B.get_model_name(),
            Model::Llama370B.get_model_name(),
        ]
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::Llama38B
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub prompt: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub current_conversation: CurrentConversation,
    pub current_model: Model,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CurrentConversation {
    pub title: Option<String>,
    /// Note that every even number is going to be a question, and every odd number is going to be an answer
    pub messages: Vec<String>,
    pub date_created: Option<i64>,
}

impl CurrentConversation {
    /// When a conversation is new, one exchange has been made. That's when we set the date and title
    pub fn is_new(&self) -> bool {
        self.messages.len() == 2 && self.date_created.is_none() && self.title.is_none()
    }

    pub fn clear(&mut self) {
        self.title = None;
        self.messages = vec![];
        self.date_created = None;
    }
}