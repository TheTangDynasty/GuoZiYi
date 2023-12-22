use std::{collections::HashMap, task::Context};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{service::route::RouteItem, utils::app_state::AppState};

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

pub async fn alive_token(state: State<AppState>) -> impl IntoResponse {
    let context = tera::Context::new();
    let html = state.tera.render("routes.tera", &context).unwrap();
    Html(html)
}

#[derive(Deserialize, Debug)]
pub struct RecordBody {
    pub uuid: String,
    pub routes: Vec<RouteItem>,
}

pub async fn record(mut state: State<AppState>, Json(body): Json<RecordBody>) -> impl IntoResponse {
    let mut routes = body.routes;
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
    let username = token.user_name;

    let mut route_names: Vec<String> = state
        .get_route_names()
        .await
        .unwrap()
        .iter()
        .map(|r| r.name.clone())
        .collect();

    let ms_stamp_limit = 1e12 as i64;

    for route in &mut routes {
        if route.commingTime > ms_stamp_limit {
            route.commingTime = route.commingTime / 1000;
        }
        if route.leaveTime > ms_stamp_limit {
            route.leaveTime = route.leaveTime / 1000;
        }
        if route.get_cost_time() > 1 {
            match state.create_route(route, &uuid, &username).await {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                }
            }
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
