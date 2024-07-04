use kinode_process_lib::{Request, println};
use llm_interface::openai::LLMResponse;
use llm_interface::openai::LLMRequest;
use llm_interface::openai::MessageBuilder;
use llm_interface::openai::ChatRequestBuilder;
use llm_interface::openai::ClaudeChatRequestBuilder;

pub const LLM_ADDRESS: (&str, &str, &str, &str) =
    ("our", "openai", "command_center", "appattacc.os");

pub fn get_groq_answer(text: &str, model: &str) -> anyhow::Result<String> {
    let request = ChatRequestBuilder::default()
        .model(model.to_string())
        .messages(vec![MessageBuilder::default()
            .role("user".to_string())
            .content(text.to_string())
            .build()?])
        .build()?;
    let request = serde_json::to_vec(&LLMRequest::GroqChat(request))?;
    let response = Request::to(LLM_ADDRESS)
        .body(request)
        .send_and_await_response(30)??;
    let LLMResponse::Chat(chat) = serde_json::from_slice(response.body())? else {
        return Err(anyhow::anyhow!("Failed to parse LLM response"));
    };
    Ok(chat.choices[0].message.content.clone())
}

pub fn get_claude_answer(text: &str, message_history: &Vec<String>, model: &str) -> anyhow::Result<String> {
    let mut messages = vec![];
    
    // We truncate the conversation based on the number of messages to not go beyond the context window. 
    // TODO: We should use some kind of tokenizer to estimate by token length.
    // We select the index so that the first message is the last user message and the last message is the last assistant message.
    let start_index = if message_history.len() > 10 {
        if message_history.len() % 2 == 0 {
            message_history.len() - 10
        } else {
            message_history.len() - 9
        }
    } else {
        0
    };

    // Make sure the message history is evenly divisible by 2 to ensure the last message is the last assistant message.
    assert_eq!(message_history.len() % 2, 0);
    for (i, message) in message_history.iter().enumerate().skip(start_index) {
        let role = if (start_index + i) % 2 == 0 { "user".to_string() } else { "assistant".to_string() };
        messages.push(MessageBuilder::default()
            .role(role)
            .content(message.to_string())
            .build()?);
    }
    messages.push(MessageBuilder::default()
        .role("user".to_string())
        .content(text.to_string())
        .build()?);

    let request = ClaudeChatRequestBuilder::default()
        .model(model.to_string())
        .messages(messages)
        .max_tokens(Some(1024))
        .build()?;
    let request = serde_json::to_vec(&LLMRequest::ClaudeChat(request))?;
    let response = Request::to(LLM_ADDRESS)
        .body(request)
        .send_and_await_response(30)??;
    let LLMResponse::ClaudeChat(chat) = serde_json::from_slice(response.body())? else {
        return Err(anyhow::anyhow!("Failed to parse LLM response"));
    };
    let message = chat.content.last().map(|c| c.text.clone()).unwrap_or_default();
    Ok(message)
}