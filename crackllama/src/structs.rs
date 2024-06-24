use kinode_process_lib::{
    get_state, set_state,
};
use serde::{Deserialize, Serialize};

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
    pub conversations: Vec<CurrentConversation>,
}

impl State {
    pub fn fetch() -> Option<State> {
        if let Some(state_bytes) = get_state() {
            bincode::deserialize(&state_bytes).ok()
        } else {
            None
        }
    }

    pub fn save(&self) {
        let serialized_state = bincode::serialize(self).expect("Failed to serialize state");
        set_state(&serialized_state);
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CurrentConversation {
    pub title: Option<String>,
    /// Note that every even number is going to be a question, and every odd number is going to be an answer
    pub messages: Vec<String>,
    pub date_created: Option<i64>,
}