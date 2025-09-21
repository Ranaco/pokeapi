use reqwest::Client;
use serde_json::Value;
use crate::errors::ApiError;
use std::time::Duration;

pub struct PokeApiClient {
    client: Client,
    base_url: String
}

impl PokeApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("RustPokemonAPI/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://pokeapi.co/api/v2".to_string(),
        }
    }

    pub async fn get_pokemon(&self, identifier: &str) -> Result<Value, ApiError> {
        let url = format!("{}/pokemon/{}", self.base_url, identifier);
        self.make_request(&url).await
    }

    pub async fn list_pokemon(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Value, ApiError> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let url = format!("{}/pokemon?limit={}&offset={}", self.base_url, limit, offset);
        self.make_request(&url).await
    }

    pub async fn get_type(&self, identifier: &str) -> Result<Value, ApiError> {
        let url = format!("{}/type/{}", self.base_url, identifier);
        self.make_request(&url).await
    }

    pub async fn list_types(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Value, ApiError> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let url = format!("{}/type?limit={}&offset={}", self.base_url, limit, offset);
        self.make_request(&url).await
    }

    pub async fn get_ability(&self, identifier: &str) -> Result<Value, ApiError> {
        let url = format!("{}/ability/{}", self.base_url, identifier);
        self.make_request(&url).await
    }

    pub async fn list_abilities(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Value, ApiError> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let url = format!("{}/ability?limit={}&offset={}", self.base_url, limit, offset);
        self.make_request(&url).await
    }

    pub async fn get_move(&self, identifier: &str) -> Result<Value, ApiError> {
        let url = format!("{}/move/{}", self.base_url, identifier);
        self.make_request(&url).await
    }

    pub async fn list_moves(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Value, ApiError> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let url = format!("{}/move?limit={}&offset={}", self.base_url, limit, offset);
        self.make_request(&url).await
    }

    pub async fn get_evolution_chain(&self, id: &str) -> Result<Value, ApiError> {
        let url = format!("{}/evolution-chain/{}", self.base_url, id);
        self.make_request(&url).await
    }

    pub async fn list_evolution_chains(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Value, ApiError> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let url = format!("{}/evolution-chain?limit={}&offset={}", self.base_url, limit, offset);
        self.make_request(&url).await
    }

    async fn make_request(&self, url: &str) -> Result<Value, ApiError> {
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| ApiError::RequestFailed(e.to_string()))?;

        match response.status().as_u16() {
            200 => {
                let json: Value = response
                    .json()
                    .await
                    .map_err(|e| ApiError::ParseError(e.to_string()))?;
                Ok(json)
            }
            404 => Err(ApiError::NotFound),
            500..=599 => Err(ApiError::UpstreamError),
            _ => Err(ApiError::RequestFailed(format!("HTTP {}", response.status())))
        }
    }
}

impl Default for PokeApiClient {
    fn default() -> Self {
        Self::new()
    }
}
