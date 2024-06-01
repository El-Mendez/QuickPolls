mod models;
mod state;
mod api;

use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::{ServeDir, ServeFile};
use socketioxide::SocketIo;
use crate::api::api_routes;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (layer, io) = SocketIo::new_layer();
    let state = AppState::new(io).await.expect("could not connect to db");
    let serve_static_files = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest_service("/api", api_routes(state))
        .fallback_service(serve_static_files)
        .layer(layer)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


