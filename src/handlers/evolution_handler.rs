use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use crate::{client::PokeApiClient, errors::ApiError};

#[derive(Deserialize)]
pub struct EvolutionListQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

pub async fn list_evolution_chains(Query(params): Query<EvolutionListQuery>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.list_evolution_chains(params.limit, params.offset).await?;
    Ok(Json(result))
}

pub async fn get_evolution_chain(Path(id): Path<String>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.get_evolution_chain(&id).await?;
    Ok(Json(result))
}
