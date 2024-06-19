use kinode_process_lib::{await_message, call_init, get_blob, http, println, Address, Message};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

mod structs;
use structs::*;

mod llm;
use llm::*;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

fn handle_message(
    our: &Address,
    current_conversation: &mut CurrentConversation,
) -> anyhow::Result<()> {
    let msg = await_message()?;
    if msg.source().node != our.node {
        return Err(anyhow::anyhow!(
            "message from {:?} is not from our node",
            msg.source()
        ));
    }
    if msg.source().process == "http_server:distro:sys" {
        return handle_http_messages(&msg, current_conversation);
    }

    Ok(())
}

fn handle_http_messages(
    msg: &Message,
    current_conversation: &mut CurrentConversation,
) -> anyhow::Result<()> {
    match msg {
        Message::Request { ref body, .. } => {
            return handle_http_request(body, current_conversation);
        }
        Message::Response { .. } => {}
    }

    Ok(())
}

fn handle_http_request(
    body: &[u8],
    current_conversation: &mut CurrentConversation,
) -> anyhow::Result<()> {
    let http_request = http::HttpServerRequest::from_bytes(body)?
        .request()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse http request"))?;
    let path = http_request.path()?;
    let bytes = get_blob()
        .ok_or_else(|| anyhow::anyhow!("Failed to get blob"))?
        .bytes;

    match path.as_str() {
        "/prompt" => prompt(&bytes, current_conversation),
        _ => Ok(()),
    }
}

fn prompt(bytes: &[u8], current_conversation: &mut CurrentConversation) -> anyhow::Result<()> {
    let prompt = serde_json::from_slice::<Prompt>(bytes)?;
    let answer = get_groq_answer(&prompt.prompt)?;

    update_conversation(&prompt.prompt, &answer, current_conversation)?;

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )])),
        answer.to_string().as_bytes().to_vec(),
    );
    Ok(())
}

fn update_conversation(prompt: &str, answer: &str, current_conversation: &mut CurrentConversation) -> anyhow::Result<()> {
    current_conversation.messages.push(prompt.to_string());
    current_conversation.messages.push(answer.to_string());

    if current_conversation.is_new() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        current_conversation.date_created = Some(now);

        let summary_prompt = format!("Given the following conversation: {:?}, summarize the topic in 80 words or less. Only output the title, do not explain yourself.", current_conversation.messages);
        let summary_answer = get_groq_answer(&summary_prompt)?;
        current_conversation.title = Some(summary_answer);
    }

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");
    if let Err(e) = http::serve_index_html(&our, "ui", false, true, vec!["/", "/prompt"]) {
        panic!("Error binding https paths: {:?}", e);
    }

    let mut current_conversation = CurrentConversation {
        title: None,
        messages: vec![],
        date_created: None,
    };

    loop {
        match handle_message(&our, &mut current_conversation) {
            Ok(()) => {}
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
