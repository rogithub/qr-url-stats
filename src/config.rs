use std::env;

#[derive(Clone)]
pub struct Config {
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let base_url = env::var("BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());
        
        Self { base_url }
    }
}