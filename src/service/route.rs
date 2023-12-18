use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    entity::{route_names, routes},
    utils::app_state::AppState,
};
use sea_orm::*;

#[derive(Deserialize, Debug)]
pub struct RouteItem {
    pub commingTime: u64,
    pub leaveTime: u64,
    pub pathname: String,
    pub url: String,
    pub route: String,
    pub searchObj: HashMap<String, String>,
}

impl RouteItem {
    pub fn get_cost_time(&self) -> i32 {
        (self.leaveTime - self.commingTime) as i32
    }
}

impl AppState {
    pub async fn create_route(
        &self,
        route: &RouteItem,
        session_id: &String,
        username: &String,
    ) -> Result<routes::Model, sea_orm::DbErr> {
        routes::ActiveModel {
            pathname: Set(route.pathname.clone()),
            route: Set(route.route.clone()),
            created_at: Set(route.commingTime as i32),
            leaved_at: Set(route.leaveTime as i32),
            cost_time: Set(route.get_cost_time()),
            url: Set(route.url.clone()),
            search_obj: Set(serde_json::to_string(&route.searchObj).unwrap()),
            username: Set(username.clone()),
            session_id: Set(session_id.clone()),
            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    pub async fn get_route_names(&self) -> Result<Vec<route_names::Model>, sea_orm::DbErr> {
        route_names::Entity::find().all(&self.db).await
    }

    pub async fn create_route_name(
        &self,
        name: &String,
    ) -> Result<route_names::Model, sea_orm::DbErr> {
        let now = chrono::Local::now().timestamp();
        route_names::ActiveModel {
            name: Set(Some(name.clone())),
            created_at: Set(Some(now as i32)),
            ..Default::default()
        }
        .insert(&self.db)
        .await
    }
}
