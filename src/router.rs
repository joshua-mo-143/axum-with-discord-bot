use axum::http::StatusCode;
use axum::{Router, routing::get};
use axum::response::IntoResponse;

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
}

pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello world!").into_response()
}