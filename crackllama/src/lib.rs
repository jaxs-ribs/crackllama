use kinode_process_lib::{await_message, call_init, println, Address, Message, http, get_blob};
use std::collections::HashMap;

mod structs;
use structs::*;

mod llm;
use llm::*;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

fn handle_message(our: &Address) -> anyhow::Result<()> {
    let msg = await_message()?;
    if msg.source().node != our.node {
        return Err(anyhow::anyhow!("message from {:?} is not from our node", msg.source()));
    }
    if msg.source().process == "http_server:distro:sys" {
        return handle_http_messages(&msg);
    }
    Ok(())
}

fn handle_http_messages(msg: &Message) -> anyhow::Result<()> {
    match msg {
        Message::Request { ref body, .. } => {
            return handle_http_request(body);
        },
        Message::Response { .. } => {

        },
    }

    Ok(())
}

fn handle_http_request(body: &[u8]) -> anyhow::Result<()> {
    let http_request = http::HttpServerRequest::from_bytes(body)?
        .request()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse http request"))?;
    let path = http_request.path()?;
    let bytes = get_blob()
        .ok_or_else(|| anyhow::anyhow!("Failed to get blob"))?
        .bytes;

    match path.as_str() {
        "/prompt" => prompt(&bytes),
        _ => Ok(()),
    }
}

fn prompt(bytes: &[u8]) -> anyhow::Result<()> {
    let prompt = serde_json::from_slice::<Prompt>(bytes)?;
    let answer = get_groq_answer(&prompt.prompt)?;
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
        ],
    ) {
        panic!("Error binding https paths: {:?}", e);
    }

    loop {
        match handle_message(&our) {
            Ok(()) => {}
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
