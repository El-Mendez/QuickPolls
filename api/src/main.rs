mod models;
mod state;
mod api;

use std::env;
use axum::Router;
use axum::routing::get;
use axum_prometheus::PrometheusMetricLayer;
use log::info;
use tracing::Level;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::{ServeDir, ServeFile};
use socketioxide::SocketIo;
use tokio::task::JoinSet;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use crate::api::api_routes;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // load config
    let public_port = env::var("PUBLIC_PORT").unwrap_or_else(|_| String::from("80"));
    let private_port = env::var("GRAFANA_PORT").unwrap_or_else(|_| String::from("3000"));
    let db_uri = env::var("DATABASE_URI").expect("Missing database URI");
    let migrate = env::var("MIGRATE").unwrap_or_else(|_| String::from("no"));
    let should_only_run_migrations = migrate == "only";
    let should_run_migrations = should_only_run_migrations || migrate == "yes" || migrate == "true";

    // load layers
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
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let state = AppState::new(io, &db_uri, should_run_migrations).await.expect("could not connect to db");
    if should_only_run_migrations {
        return;
    }

    // load apps
    let public_app = Router::new()
        .nest_service("/api", api_routes(state))
        .fallback_service(serve_static_files)
        .layer(socket_io_layer)
        .layer(prometheus_layer)
        .layer(logging_layer)
        .layer(cors);
    let private_app = Router::new()
        .route("/metrics", get(|| async move { metric_handle.render() }));

    let mut public_addr = String::from("0.0.0.0:");
    public_addr.push_str(&public_port);
    let mut private_addr = String::from("0.0.0.0:");
    private_addr.push_str(&private_port);

    let public_listener = tokio::net::TcpListener::bind(public_addr.clone()).await.unwrap();
    let private_listener = tokio::net::TcpListener::bind(private_addr.clone()).await.unwrap();

    info!("Server up on {public_addr}");
    let public_server = axum::serve(public_listener, public_app);
    let private_server = axum::serve(private_listener, private_app);

    let mut set = JoinSet::new();
    set.spawn(tokio::spawn(async { public_server.await.unwrap() } ));
    set.spawn(tokio::spawn(async { private_server.await.unwrap() } ));
    let _ = set.join_next().await.unwrap().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
