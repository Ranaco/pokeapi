use axum::{extract::{Path, Query}, Json};
use serde::Deserialize;
use serde_json::Value;
use crate::{client::PokeApiClient, errors::ApiError};

#[derive(Deserialize)]
pub struct PokemonListQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

pub async fn list_pokemon(Query(params): Query<PokemonListQuery>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.list_pokemon(params.limit, params.offset).await?;
    Ok(Json(result))
}

pub async fn get_pokemon(Path(identifier): Path<String>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.get_pokemon(&identifier).await?;
    Ok(Json(result))
}
