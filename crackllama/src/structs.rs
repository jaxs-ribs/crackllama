use serde::{Deserialize, Serialize};
use storage_interface::CurrentConversation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    Llama38B,
    Llama370B,
}

impl Model {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Model::Llama38B,
            1 => Model::Llama370B,
            _ => panic!("Invalid model index"),
        }
    }
    
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

