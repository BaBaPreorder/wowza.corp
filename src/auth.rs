use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: u64,
}

pub fn authenticate(
    client: &Client,
    api_url: &str,
    username: &str,
    password: &str,
) -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let login_url = format!("{}/auth/login", api_url);
    
    let request_body = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = client
        .post(&login_url)
        .json(&request_body)
        .timeout(Duration::from_secs(30))
        .send()?;

    if response.status().is_success() {
        let login_resp: LoginResponse = response.json()?;
        Ok(login_resp)
    } else {
        Err(format!("Authentication failed: {}", response.status()).into())
    }
}
