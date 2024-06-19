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
    pub date_created: Option<i64>,
}

impl CurrentConversation {
    /// When a conversation is new, one exchange has been made. That's when we set the date and title
    pub fn is_new(&self) -> bool {
        self.messages.len() == 2 && self.date_created.is_none() && self.title.is_none()
    }

    pub fn _clear(&mut self) {
        self.title = None;
        self.messages = vec![];
        self.date_created = None;
    }
}