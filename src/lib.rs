use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const VALID_USERNAME: &str = "abc";
const VALID_PASSWORD: &str = "pass";

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| handler(headers, qry, body)).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let login_request: LoginRequest = match serde_json::from_slice(&body) {
        Ok(login_request) => login_request,
        Err(_) => {
            // Respond with a failure message if the request body is not a valid JSON or cannot be deserialized
            let resp = "Invalid JSON request body";
            send_response(
                400, // Bad Request status code
                vec![(String::from("content-type"), String::from("text/html"))],
                resp.as_bytes().to_vec(),
            );
            return;
        }
    };

    if let (Some(username), Some(password)) =
        (Some(login_request.username), Some(login_request.password))
    {
        // Validate the provided credentials against predefined credentials
        if username == VALID_USERNAME && password == VALID_PASSWORD {
            // Login successful
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
}
