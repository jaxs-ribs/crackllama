use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{await_message, call_init, println, Address, ProcessId, Request, Response, http};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

fn handle_message(our: &Address) -> anyhow::Result<()> {
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
