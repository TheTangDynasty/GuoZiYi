use std::collections::HashMap;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{entity::routes, service::route::RouteItem, utils::app_state::AppState};

#[derive(Deserialize)]
pub struct CreateTokenBody {
    username: String,
}

pub async fn create_token(
    mut state: State<AppState>,
    Json(body): Json<CreateTokenBody>,
) -> impl IntoResponse {
    let username = body.username;
    match state.create_token(username).await {
        Ok(token) => (
            StatusCode::OK,
            Json(json!({
                "data": token,
                "code": 0,
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "msg": e.to_string(),
                "code": -1,
            })),
        ),
    }
}

pub async fn alive_token(mut state: State<AppState>) -> impl IntoResponse {
    let tokens = state.get_alive_tokens().await;
    Json(json!({
        "data": tokens,
        "code": 0
    }))
}

#[derive(Deserialize, Debug)]
pub struct RecordBody {
    pub uuid: String,
    pub routes: Vec<RouteItem>,
}

pub async fn record(mut state: State<AppState>, Json(body): Json<RecordBody>) -> impl IntoResponse {
    let routes = body.routes;
    let uuid = body.uuid;

    let token = match state.get_token(uuid.clone()).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Json(json!({
                "data": format!("uuid {} is not exist", uuid.clone()),
                "code": 1
            }))
        }
        Err(e) => {
            return Json(json!({
                "data": format!("{}", e.to_string()),
                "code": -1
            }))
        }
    };
    let username = token.user_name.unwrap();

    let mut route_names: Vec<String> = state
        .get_route_names()
        .await
        .unwrap()
        .iter()
        .map(|r| r.name.clone().unwrap())
        .collect();

    for route in routes.iter() {
        if route.get_cost_time() > 1000 {
            let _ = state.create_route(route, &uuid, &username).await;
            if !route_names.contains(&route.route) {
                route_names.push(route.route.clone());
                let _ = state.create_route_name(&route.route).await;
            }
        }
    }

    Json(json!({
        "data": 0,
        "code": 0
    }))
}
