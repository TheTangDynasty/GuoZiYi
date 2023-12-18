use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use serde_json::json;

use crate::utils::app_state::AppState;

pub async fn attach_heart_beat(
    mut state: State<AppState>,
    Query(mut queries): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match queries.remove("token") {
        Some(v) => v,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"code": -1, "message": "query is not exist"})),
            );
        }
    };
    match state.update_token(token).await {
        Ok(_) => {}
        Err(e) => {
            return (
                StatusCode::OK,
                Json(json!({"code": -1, "message": e.to_string()})),
            );
        }
    };
    (StatusCode::OK, Json(json!({"code": 0})))
}
