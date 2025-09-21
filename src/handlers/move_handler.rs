use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use crate::{client::PokeApiClient, errors::ApiError};

#[derive(Deserialize)]
pub struct MoveListQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

pub async fn list_moves(Query(params): Query<MoveListQuery>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.list_moves(params.limit, params.offset).await?;
    Ok(Json(result))
}

pub async fn get_move(Path(identifier): Path<String>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.get_move(&identifier).await?;
    Ok(Json(result))
}
