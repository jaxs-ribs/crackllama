use kinode_process_lib::{
    get_state, set_state,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub conversations: HashMap<i32, Conversation>,
}

impl State {
    pub fn fetch() -> Option<State> {
        if let Some(state_bytes) = get_state() {
            bincode::deserialize(&state_bytes).ok()
        } else {
            None
        }
    }

    pub fn save(&mut self) {
        // Remove conversations with no messages
        self.conversations.retain(|_, conversation| !conversation.messages.is_empty());
        let serialized_state = bincode::serialize(self).expect("Failed to serialize state");
        set_state(&serialized_state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub title: Option<String>,
    /// Note that every even number is going to be a question, and every odd number is going to be an answer
    pub messages: Vec<String>,
    pub date_created: i64,
}

impl Default for Conversation {
    fn default() -> Self {
        let date_created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        Self {
            title: None,
            messages: vec![],
            date_created,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    Llama38B,
    Llama370B,
}

impl Model {
    pub fn _from_index(index: usize) -> Self {
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
    pub conversation_id: i32,
    pub model: String,
    pub prompt: String,
    pub enriched_prompt: Option<String>,
}
