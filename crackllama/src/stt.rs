use kinode_process_lib::{
    await_message, call_init, Address, Message, get_blob, Request, println 
};

use stt_interface::STTRequest;
use stt_interface::STTResponse;

pub const STT_ADDRESS: (&str, &str, &str, &str) =
    ("our", "speech_to_text", "command_center", "appattacc.os");

pub fn get_text(audio: Vec<u8>) -> anyhow::Result<String> {
    println!("chatbot: get text");
    let stt_request = serde_json::to_vec(&STTRequest::OpenaiTranscribe(audio))?;
    let response = Request::to(STT_ADDRESS)
        .body(stt_request)
        .send_and_await_response(3)??;
    let response_body = String::from_utf8(response.body().to_vec())?;
    println!("STT response body: {}", response_body);
    let STTResponse::OpenaiTranscribed(text) = serde_json::from_slice(response.body())? else {
        println!("chatbot: failed to parse STT response");
        return Err(anyhow::anyhow!("Failed to parse STT response"));
    };
    Ok(text)
}