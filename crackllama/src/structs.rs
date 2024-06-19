use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentConversation {
    pub title: Option<String>,
    /// Note that every even number is going to be a question, and every odd number is going to be an answer
    pub messages: Vec<String>,
}