use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

fn main() {
    
    let api_url = "https://api.company.internal/auth/login";
    let username = "admin@company.com";
    let password = "**Will Add Later**";  

    let client = Client::new();
    let login_data = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    match client.post(api_url).json(&login_data).send() {
        Ok(response) => {
            println!("Status: {}", response.status());
            if let Ok(text) = response.text() {
                println!("Response: {}", text);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
