mod models;
mod state;
mod api;

use std::env;
use axum::Router;
use log::info;
use tracing::Level;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::{ServeDir, ServeFile};
use socketioxide::SocketIo;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use crate::api::api_routes;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let db_uri = env::var("DATABASE_URI").expect("Missing database URI");
    let run_migrations = env::args().find(|x| x == "--run-migrations").is_some();

    let (socket_io_layer, io) = SocketIo::new_layer();
    let serve_static_files = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let logging_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let state = AppState::new(io, &db_uri, run_migrations).await.expect("could not connect to db");
    let app = Router::new()
        .nest_service("/api", api_routes(state))
        .fallback_service(serve_static_files)
        .layer(socket_io_layer)
        .layer(logging_layer)
        .layer(cors);

    let mut addr: String = "0.0.0.0:".into();
    addr.push_str(&port);

    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
    info!("Server up on {addr}");
    axum::serve(listener, app).await.unwrap();
}


