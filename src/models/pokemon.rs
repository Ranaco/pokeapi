use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub base_experience: Option<u32>,
    pub is_default: bool,
    pub order: Option<u32>,
    // Add other fields as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonList {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<NamedApiResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}
