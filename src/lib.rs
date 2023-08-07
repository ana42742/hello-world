use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde_json::Value;

const VALID_USERNAME: &str = "abc";
const VALID_PASSWORD: &str = "pass";

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        handler(headers, qry, body)
    }).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    // let msg = qry.get("msg").unwrap();
    let username = qry.get("username").unwrap();
    let password = qry.get("password").unwrap();

    if let (Some(username), Some(password)) = (username, password) {
        // Validate the provided credentials against predefined credentials
        if username == VALID_USERNAME && password == VALID_PASSWORD {
            let resp = format!("Login successful for user: {}", username);
            send_response(
                200,
                vec![(String::from("content-type"), String::from("text/html"))],
                resp.as_bytes().to_vec(),
            );
            return;
        }
    }

    let resp = "Login failed: Invalid username or password";
    send_response(
        401, // Unauthorized status code
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.as_bytes().to_vec(),
    );

    // let msg = String::from_utf8(body).unwrap_or("".to_string());
    // let resp = format!("Welcome to flows.network.\nYou just said: '{}'.\nLearn more at: https://github.com/flows-network/hello-world\n", msg);

    // send_response(
    //     200,
    //     vec![(String::from("content-type"), String::from("text/html"))],
    //     resp.as_bytes().to_vec(),
    // );
}
