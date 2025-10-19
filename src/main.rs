mod config;
mod auth;
mod api;

use reqwest::blocking::Client;
use std::time::Duration;

fn main() {
    println!("Company API Client v0.2.1");
    println!("Initializing...\n");


    let config = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            eprintln!("Please set API_USERNAME and API_PASSWORD environment variables.");
            return;
        }
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .build()
        .expect("Failed to create HTTP client");

    println!("Authenticating to {}...", config.api_url);
    
    match auth::authenticate(&client, &config.api_url, &config.username, &config.password) {
        Ok(login_resp) => {
            println!("✓ Authentication successful!");
            println!("Token expires in: {} seconds\n", login_resp.expires_in);

            println!("Fetching user data...");
            match api::fetch_user_data(&client, &config.api_url, &login_resp.token) {
                Ok(users) => {
                    println!("✓ Found {} users:", users.len());
                    for user in users {
                        println!("  - {} ({}): {}", user.id, user.role, user.email);
                    }
                }
                Err(e) => eprintln!("✗ Failed to fetch users: {}", e),
            }
        }
        Err(e) => {
            eprintln!("✗ Authentication failed: {}", e);
        }
    }
}
