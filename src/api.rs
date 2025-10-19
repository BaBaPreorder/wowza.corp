use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub id: u32,
    pub email: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct DataRequest {
    pub query: String,
}

pub fn fetch_user_data(
    client: &Client,
    api_url: &str,
    token: &str,
) -> Result<Vec<UserData>, Box<dyn std::error::Error>> {
    let users_url = format!("{}/api/users", api_url);
    
    let response = client
        .get(&users_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    if response.status().is_success() {
        let users: Vec<UserData> = response.json()?;
        Ok(users)
    } else {
        Err(format!("Failed to fetch users: {}", response.status()).into())
    }
}
