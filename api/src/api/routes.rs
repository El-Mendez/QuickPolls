use std::default::Default;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use chrono::Utc;
use log::{error, info};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use crate::api::controller::PollController;
use crate::api::socket::socket_handler;
use crate::models::{poll, poll_option};
use crate::state::AppState;

type Result<T> = core::result::Result<T, (StatusCode, String)>;

pub fn api_routes(state: AppState) -> Router {
    state.io.ns("/", socket_handler);

    Router::new()
        .route("/polls", post(create_poll))
        .route("/polls/:poll_id", get(get_poll).post(vote_poll))
        .route("/polls/:poll_id/results", get(poll_results))
        .route("/polls/:poll_id/end", post(end_poll))
        .with_state(state)
}


#[derive(Deserialize)]
struct CreatePoll {
    title: String,
    options: Vec<String>,
}

async fn create_poll(state: State<AppState>, data: Json<CreatePoll>) -> Result<Json<poll::Model>> {
    let data = data.0;
    let poll: poll::ActiveModel = poll::ActiveModel {
        title: Set(data.title),
        start_date: Set(Utc::now().into()),
        ..Default::default()
    };

    let options: Vec<poll_option::ActiveModel> = data.options.into_iter().map(|option| {
        poll_option::ActiveModel {
            value: Set(option),
            ..Default::default()
        }
    }).collect();

    PollController::create_poll(&state.db, poll, options)
        .await
        .map_err(|e| {
            error!("{e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not create poll".into())
        })
        .map(Json)
}


async fn get_poll(state: State<AppState>, Path(id): Path<i32>) -> Result<Json<Value>> {
    PollController::get_poll(&state.db, id)
        .await
        .map_err(|e| {
            error!("{e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not get poll".into())
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Not found".into()))
        .map(|(poll, options)| {
            let mut js: Map<String, Value> = match json!(poll) {
                Value::Object(x) => x,
                _ => unreachable!(),
            };

            js.insert("options".into(), json!(options));
            Json(Value::Object(js))
        })
}

#[derive(Deserialize, Debug)]
struct VotePollAnswer {
    answer_id: i32,
}

async fn vote_poll(state: State<AppState>, Path(id): Path<i32>, Json(vote): Json<VotePollAnswer>) -> Result<Json<Value>> {
    match PollController::vote_poll(&state.db, id, vote.answer_id).await {
        Err(e) => {
            error!("{e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not vote poll".into()))
        },
        Ok(x) => {
            let emitted = serde_json::to_value(&x).unwrap();
            info!("emitted {emitted}");
            let _ = state.io.to(id.to_string()).emit("vote", emitted);
            Ok(Json(serde_json::to_value(x).unwrap()))
        }
    }
}

async fn poll_results(state: State<AppState>, Path(id): Path<i32>) -> Result<Json<Value>> {
    PollController::get_results(&state.db, id)
        .await
        .map_err(|e| {
            error!("{e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Could not find poll results".into())
        })
        .map(|x| serde_json::to_value(x).unwrap())
        .map(Json)
}

async fn end_poll(state: State<AppState>, Path(id): Path<i32>) -> Result<StatusCode> {
    match PollController::end_poll(&state.db, id).await {
        Err(e) => {
            error!("{e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "could not end poll".into()))
        },
        Ok(None) => Err((StatusCode::NOT_FOUND, "Could not find poll".into())),
        Ok(Some(())) => {
            let _ = state.io.to(id.to_string()).emit("end", ());
            Ok(StatusCode::OK)
        }
    }
}
