use axum::{Router, routing::get};
use crate::handlers::{pokemon_handler, type_handler, ability_handler, move_handler, evolution_handler};

pub async fn create_router() -> Router {
    Router::new()
        .route("/api/v2/pokemon", get(pokemon_handler::list_pokemon))
    .route("/api/v2/pokemon/{identifier}", get(pokemon_handler::get_pokemon))

        .route("/api/v2/type", get(type_handler::list_types))
    .route("/api/v2/type/{identifier}", get(type_handler::get_type))

        .route("/api/v2/ability", get(ability_handler::list_abilities))
    .route("/api/v2/ability/{identifier}", get(ability_handler::get_ability))

        .route("/api/v2/move", get(move_handler::list_moves))
    .route("/api/v2/move/{identifier}", get(move_handler::get_move))

        .route("/api/v2/evolution-chain", get(evolution_handler::list_evolution_chains))
    .route("/api/v2/evolution-chain/{id}", get(evolution_handler::get_evolution_chain))

        .route("/health", get(|| async { "OK" }))
}
