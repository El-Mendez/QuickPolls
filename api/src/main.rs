mod models;
mod state;
mod api;

use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use socketioxide::SocketIo;
use crate::api::api_routes;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (layer, io) = SocketIo::new_layer();
    let state = AppState::new(io).await.expect("could not connect to db");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest_service("/api", api_routes(state))
        .route("/", get(root))
        .layer(layer)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}

