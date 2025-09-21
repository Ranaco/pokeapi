use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use crate::{client::PokeApiClient, errors::ApiError};

#[derive(Deserialize)]
pub struct AbilityListQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

pub async fn list_abilities(Query(params): Query<AbilityListQuery>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.list_abilities(params.limit, params.offset).await?;
    Ok(Json(result))
}

pub async fn get_ability(Path(identifier): Path<String>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.get_ability(&identifier).await?;
    Ok(Json(result))
}
