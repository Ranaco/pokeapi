use axum::http::{Request, StatusCode};
use axum::body::Body;
use tower::ServiceExt;
use crate::routes;

#[tokio::test]
async fn test_get_pokemon() {
    let app = routes::create_router().await;

    let response = app
        .oneshot(Request::builder().uri("/api/v2/pokemon/pikachu").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_pokemon() {
    let app = routes::create_router().await;

    let response = app
        .oneshot(Request::builder().uri("/api/v2/pokemon").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_check() {
    let app = routes::create_router().await;

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
