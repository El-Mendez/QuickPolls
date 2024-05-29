use std::default::Default;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use crate::api::controller::PollController;
use crate::models::poll;
use crate::state::AppState;

type Result<T> = core::result::Result<T, (StatusCode, String)>;

pub fn api_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_poll))
        .route("/:poll_id", get(get_poll))
        .route("/:poll_id/results", get(poll_results))
        .with_state(state)
}


#[derive(Deserialize)]
struct CreatePoll {
    title: String,
}

async fn create_poll(state: State<AppState>, data: Json<CreatePoll>) -> Result<Json<poll::Model>> {
    let data = data.0;
    let poll: poll::ActiveModel = poll::ActiveModel {
        title: Set(data.title),
        start_date: Set(Utc::now().into()),
        ..Default::default()
    };

    PollController::create_poll(&state.db, poll)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not create poll".into()))
        .map(Json)
}

async fn get_poll(state: State<AppState>, Path(id): Path<u32>) -> Result<Json<poll::Model>> {
    PollController::get_poll(&state.db, id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not get poll".into()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Not found".into()))
        .map(Json)
}

async fn poll_results(state: State<AppState>, Path(id): Path<u32>) -> String {
    println!("{:?}", state.db);
    "Hello, Poll!!".into()
}

