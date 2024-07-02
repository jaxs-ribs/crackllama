use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, Message, Request,
};
use rand;
use std::collections::HashMap;

mod structs;
use structs::*;

mod llm;
use llm::*;

mod stt;
use stt::*;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

pub const VECTORBASE_ADDRESS: (&str, &str, &str, &str) =
    ("our", "vectorbase", "command_center", "appattacc.os");
pub const VECTORBASE_DATABASE_NAME: &str = "llm_conversations";

fn update_conversation(
    prompt: &str,
    answer: &str,
    conversation: &mut Conversation,
) -> anyhow::Result<()> {
    conversation.messages.push(prompt.to_string());
    conversation.messages.push(answer.to_string());

    if conversation.messages.len() == 2 {
        let summary_prompt = format!("Given the following conversation: {:?}, summarize the topic in 80 words or less. Only output the title, do not explain yourself.", conversation.messages);
        let summary_answer = get_groq_answer(&summary_prompt, &Model::Llama38B.get_model_name())?;
        conversation.title = Some(summary_answer);
    }

    Ok(())
}

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

fn prompt(bytes: &[u8], state: &mut State) -> anyhow::Result<()> {
    let prompt = serde_json::from_slice::<Prompt>(bytes)?;
    let Some(conversation) = state.conversations.get_mut(&prompt.conversation_id) else {
        println!("Our available conversations are {:?}", state.conversations);
        println!(
            "But we got a prompt with conversation_id {:?}",
            prompt.conversation_id
        );
        http::send_response(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])),
            serde_json::to_vec(&serde_json::json!({"error": "Conversation not found"}))?,
        );
        return Ok(());
    };

    let answer = get_groq_answer_with_history(
        &prompt.prompt,
        &conversation.messages.clone(),
        &prompt.model,
    )?;

    update_conversation(&prompt.prompt, &answer, conversation)?;

    let message_history = serde_json::to_string(&conversation.messages)?;

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )])),
        message_history.clone().as_bytes().to_vec(),
    );

    {
        let conversation_id_str = prompt.conversation_id.to_string();
        let request = vectorbase_interface::vectorbase::Request::SubmitData {
            database_name: VECTORBASE_DATABASE_NAME.to_string(),
            values: vec![(conversation_id_str, message_history)],
        };

        let response = Request::to(VECTORBASE_ADDRESS)
            .body(serde_json::to_vec(&request).unwrap())
            .send_and_await_response(30)
            .unwrap()
            .unwrap();
    }

    state.save();
    Ok(())
}

fn list_models() -> anyhow::Result<()> {
    let models = Model::available_models();
    let json = serde_json::to_string(&models)?;

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )])),
        json.as_bytes().to_vec(),
    );
    Ok(())
}

fn new_conversation(state: &mut State) -> anyhow::Result<()> {
    let new_id = loop {
        let id = rand::random::<i32>();
        if !state.conversations.contains_key(&id) {
            break id;
        }
    };

    let new_conversation = Conversation::default();
    state.conversations.insert(new_id, new_conversation);

    let response = serde_json::json!({
        "id": new_id
    });

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )])),
        serde_json::to_vec(&response)?,
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

fn list_conversations(state: &State) -> anyhow::Result<()> {
    let mut conversations: Vec<_> = state
        .conversations
        .iter()
        .filter(|(_, conv)| !conv.messages.is_empty())
        .map(|(id, conv)| {
            (
                id,
                conv.title.clone().unwrap_or_default(),
                conv.date_created,
            )
        })
        .collect();

    conversations.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by date_created in descending order

    let response = conversations
        .into_iter()
        .map(|(id, title, _)| {
            serde_json::json!({
                "id": id,
                "title": title
            })
        })
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&response)?;

    http::send_response(
        http::StatusCode::OK,
        Some(HashMap::from([(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )])),
        json.as_bytes().to_vec(),
    );
    Ok(())
}

fn get_conversation(bytes: &[u8], state: &mut State) -> anyhow::Result<()> {
    let conversation_id: i32 = serde_json::from_slice(bytes)?;

    if let Some(conversation) = state.conversations.get(&conversation_id) {
        http::send_response(
            http::StatusCode::OK,
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])),
            serde_json::to_vec(&conversation.messages)?,
        );
    } else {
        http::send_response(
            http::StatusCode::NOT_FOUND,
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])),
            serde_json::to_vec(&serde_json::json!({"error": "Conversation not found"}))?,
        );
    }
    Ok(())
}

fn delete_conversation(bytes: &[u8], state: &mut State) -> anyhow::Result<()> {
    let conversation_id: i32 = serde_json::from_slice(bytes)?;

    if state.conversations.remove(&conversation_id).is_some() {
        http::send_response(
            http::StatusCode::OK,
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])),
            serde_json::to_vec(&serde_json::json!({"success": true}))?,
        );
    } else {
        http::send_response(
            http::StatusCode::NOT_FOUND,
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])),
            serde_json::to_vec(&serde_json::json!({"error": "Conversation not found"}))?,
        );
    }
    Ok(())
}

fn search(bytes: &[u8], _state: &mut State) -> anyhow::Result<()> {
    let query: String = serde_json::from_slice(bytes)?;
    println!("Searching for {:?}", query);

    let request = vectorbase_interface::vectorbase::Request::SemanticSearch {
        database_name: VECTORBASE_DATABASE_NAME.to_string(),
        top_k: 50,
        query,
    };

    let response = Request::to(VECTORBASE_ADDRESS)
        .body(serde_json::to_vec(&request).unwrap())
        .send_and_await_response(30)
        .unwrap()
        .unwrap();
    println!("Request sent!");

    if let Ok(vectorbase_interface::vectorbase::Response::SemanticSearch(results)) =
        serde_json::from_slice(
            response.body(),
    ) {
        let ids: Vec<i32> = results.iter().map(|r| r.0.parse::<i32>().unwrap_or_default()).collect();
        println!("Results: {:?}", ids);
        http::send_response(
            http::StatusCode::OK,
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])),
            serde_json::to_vec(&ids)?,
        );

        return Ok(());
    }

    return Err(anyhow::anyhow!("Failed to parse search results"));
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
        "/list_models" => list_models(),
        "/new_conversation" => new_conversation(state),
        "/prompt" => prompt(&bytes, state),
        "/transcribe" => transcribe(bytes),
        "/list_conversations" => list_conversations(state),
        "/get_conversation" => get_conversation(&bytes, state),
        "/delete_conversation" => delete_conversation(&bytes, state),
        "/search" => search(&bytes, state),
        _ => Ok(()),
    }
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
            "/list_models",
            "/new_conversation",
            "/prompt",
            "/transcribe",
            "/list_conversations",
            "/get_conversation",
            "/delete_conversation",
            "/search",
        ],
    ) {
        panic!("Error binding https paths: {:?}", e);
    }

    let mut state = State::fetch().unwrap_or_default();

    // temp_test();

    loop {
        match handle_message(&our, &mut state) {
            Ok(()) => {}
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
