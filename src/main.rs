mod models;
mod state;
mod api;

use axum::{Router, routing::get};
use crate::api::api_routes;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let state = AppState::new().await.expect("could not connect to db");

    let app = Router::new()
        .nest_service("/api", api_routes(state))
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}

