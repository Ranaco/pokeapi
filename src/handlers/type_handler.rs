use axum::{
    extract::{Path, Query},
    Json
};

use serde::Deserialize;
use serde_json::Value;
use crate::{client::PokeApiClient, errors::ApiError};

#[derive(Deserialize)]
pub struct TypeListQuery {
    limit: Option<u32>,
    offset: Option<u32>
}

pub async fn list_types(Query(query): Query<TypeListQuery>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.list_types(query.limit, query.offset).await?;

    Ok(Json(result))
}

pub async fn get_type(Path(identifier): Path<String>) -> Result<Json<Value>, ApiError> {
    let client = PokeApiClient::new();
    let result = client.get_type(&identifier).await?;

    Ok(Json(result))
}
