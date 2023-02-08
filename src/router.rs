use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

pub async fn set_up_router(addr: std::net::SocketAddr) -> Result<(), hyper::Error> {
    let router = router();

    let server = axum::Server::bind(&addr).serve(router.into_make_service());

    server.await
}

pub fn router() -> Router {
    let rtr = Router::new().route("/", get(hello_world));

    rtr
}

pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello world!").into_response()
}
