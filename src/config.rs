use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_url: String,
    pub username: String,
    pub password: String,
    pub timeout_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let api_url = env::var("API_URL")
            .unwrap_or_else(|_| "https://api.company.internal".to_string());
        
        let username = env::var("API_USERNAME")
            .map_err(|_| "API_USERNAME not set".to_string())?;
        
        let password = env::var("API_PASSWORD")
            .map_err(|_| "API_PASSWORD not set".to_string())?;
        
        let timeout_secs = env::var("API_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);

        Ok(Config {
            api_url,
            username,
            password,
            timeout_secs,
        })
    }

    pub fn default_hardcoded() -> Self {
        Config {
            api_url: "https://api.company.internal".to_string(),
            username: "admin@wowza.corp".to_string(),
            password: "**REACTED**".to_string(),  
            timeout_secs: 30,
        }
    }
}
