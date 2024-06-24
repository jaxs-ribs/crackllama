use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, Message, Request,
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

mod structs;
use structs::*;

mod llm;
use llm::*;

mod stt;
use stt::*;

mod temp;
use temp::*;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

pub const VECTORBASE_ADDRESS: (&str, &str, &str, &str) =
    ("our", "vectorbase", "command_center", "appattacc.os");

// TODO: These are helper functions
pub fn is_new(current_conversation: &CurrentConversation) -> bool {
    current_conversation.messages.len() == 2
        && current_conversation.date_created.is_none()
        && current_conversation.title.is_none()
}

pub fn clear(current_conversation: &mut CurrentConversation) {
    current_conversation.title = None;
    current_conversation.messages = vec![];
    current_conversation.date_created = None;
}

fn update_conversation(
    prompt: &str,
    answer: &str,
    current_conversation: &mut CurrentConversation,
) -> anyhow::Result<()> {
    current_conversation.messages.push(prompt.to_string());
    current_conversation.messages.push(answer.to_string());

    if is_new(current_conversation) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        current_conversation.date_created = Some(now);

        let summary_prompt = format!("Given the following conversation: {:?}, summarize the topic in 80 words or less. Only output the title, do not explain yourself.", current_conversation.messages);
        let summary_answer = get_groq_answer(&summary_prompt, &Model::Llama38B.get_model_name())?;
        current_conversation.title = Some(summary_answer);
    }

    Ok(())
}
// TODO: /endtodo

fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<()> {
    let msg = await_message()?;
    if msg.source().node != our.node {
        return Err(anyhow::anyhow!(
            "message from {:?} is not from our node",
            msg.source()
        ));
    }
    if msg.source().process == "http_server:distro:sys" {
        return handle_http_messages(&msg, state);
    }

    Ok(())
}

fn handle_http_messages(msg: &Message, state: &mut State) -> anyhow::Result<()> {
    match msg {
        Message::Request { ref body, .. } => {
            return handle_http_request(body, state);
        }
        Message::Response { .. } => {}
    }

    Ok(())
}

fn handle_http_request(body: &[u8], state: &mut State) -> anyhow::Result<()> {
    let http_request = http::HttpServerRequest::from_bytes(body)?
        .request()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse http request"))?;
    let path = http_request.path()?;
    let bytes = get_blob()
        .ok_or_else(|| anyhow::anyhow!("Failed to get blob"))?
        .bytes;

    match path.as_str() {
        "/prompt" => prompt(&bytes, state),
        "/list_models" => list_models(),
        "/set_model" => set_model(&bytes, &mut state.current_model),
        "/transcribe" => transcribe(bytes),
        "/save_conversation" => save_conversation(&mut state.current_conversation),
        _ => Ok(()),
    }
}

fn set_model(bytes: &[u8], current_model: &mut Model) -> anyhow::Result<()> {
    let index = serde_json::from_slice::<usize>(bytes)?;
    *current_model = Model::from_index(index);

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )])),
        "success".to_string().as_bytes().to_vec(),
    );
    Ok(())
}

fn prompt(bytes: &[u8], state: &mut State) -> anyhow::Result<()> {
    let prompt = serde_json::from_slice::<Prompt>(bytes)?;
    let answer = get_groq_answer_with_history(
        &prompt.prompt,
        &state.current_conversation.messages,
        &state.current_model.get_model_name(),
    )?;

    update_conversation(&prompt.prompt, &answer, &mut state.current_conversation)?;

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

fn list_models() -> anyhow::Result<()> {
    let models = Model::available_models();
    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )])),
        serde_json::to_vec(&models)?,
    );
    Ok(())
}

fn transcribe(bytes: Vec<u8>) -> anyhow::Result<()> {
    let transcript = get_text(bytes)?;

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "text/plain".to_string(),
        )])),
        transcript.as_bytes().to_vec(),
    );
    Ok(())
}

fn save_conversation(current_conversation: &mut CurrentConversation) -> anyhow::Result<()> {
    let Some(title) = &current_conversation.title else {
        return Err(anyhow::anyhow!("No title found"));
    };

    let joined_messages = current_conversation.messages.join("\n");
    let values: Vec<(String, String)> = vec![(title.clone(), joined_messages)];

    let request = vectorbase_interface::Request::SubmitData {
        database_name: "crackllama".to_string(),
        values,
    };
    let response = Request::to(VECTORBASE_ADDRESS)
        .body(serde_json::to_vec(&request).unwrap())
        .send_and_await_response(8)??;

    let status = if let vectorbase_interface::Response::SubmitData =
        serde_json::from_slice(&response.body())?
    {
        "Success".to_string()
    } else {
        "Failed to save conversation".to_string()
    };

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "text/plain".to_string(),
        )])),
        status.as_bytes().to_vec(),
    );
    clear(current_conversation);
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    println!("begin");
    if let Err(e) = http::serve_index_html(
        &our,
        "ui",
        false,
        true,
        vec![
            "/",
            "/prompt",
            "/list_models",
            "/set_model",
            "/transcribe",
            "/save_conversation",
        ],
    ) {
        panic!("Error binding https paths: {:?}", e);
    }

    let mut state = State::default();

    temp_test();

    loop {
        match handle_message(&our, &mut state) {
            Ok(()) => {}
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
