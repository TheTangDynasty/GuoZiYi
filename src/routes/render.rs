use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

use crate::utils::{app_state::AppState, get_seven_day::get_seven_day};

pub async fn main_page(mut state: State<AppState>) -> impl IntoResponse {
    let mut context = tera::Context::new();
    let route_names = state.get_route_names().await.unwrap();
    let mut route_count_map: HashMap<String, u64> = HashMap::new();
    for route_name in &route_names {
        let name = route_name.name.clone();
        let count = state.get_route_name_count(&name).await.unwrap();
        route_count_map.insert(name, count);
    }

    context.insert("route_names", &route_names);
    context.insert("route_count_map", &route_count_map);

    let _ = state.tera.full_reload();
    let html = state.tera.render("routes.tera", &context).unwrap();
    Html(html)
}

#[derive(Serialize)]
pub struct SevenDayDataItem {
    pub date_offset: i64,
    pub count: u64,
}
pub async fn route_page(
    Path(route_name): Path<String>,
    mut state: State<AppState>,
) -> impl IntoResponse {
    println!("{:?}", route_name);
    let mut context = tera::Context::new();

    let seven_day = get_seven_day();

    let mut seven_day_data: Vec<SevenDayDataItem> = Vec::new();
    for item in &seven_day {
        let count = state
            .get_routes_count_by_route_and_date(item.start_date, item.end_date)
            .await
            .unwrap();
        seven_day_data.push(SevenDayDataItem {
            date_offset: item.offset,
            count,
        });
    }

    context.insert("route_name", &route_name);
    context.insert("seven_day_data", &seven_day_data);

    let _ = state.tera.full_reload();
    let html = state.tera.render("routeitem.tera", &context).unwrap();
    Html(html)
}
