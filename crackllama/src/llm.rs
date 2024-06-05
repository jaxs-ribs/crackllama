use kinode_process_lib::{Request, println};
use llm_interface::openai::LLMResponse;
use llm_interface::openai::LLMRequest;
use llm_interface::openai::MessageBuilder;
use llm_interface::openai::ChatRequestBuilder;

pub const LLM_ADDRESS: (&str, &str, &str, &str) =
    ("our", "openai", "command_center", "appattacc.os");

pub fn get_groq_answer(text: &str) -> anyhow::Result<String> {
    let request = ChatRequestBuilder::default()
        .model("llama3-8b-8192".to_string())
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