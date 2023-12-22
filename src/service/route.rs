use std::collections::HashMap;

use serde::Deserialize;

use ::entity::{route_names, routes};

use crate::utils::app_state::AppState;
use sea_orm::*;

#[derive(Deserialize, Debug)]
pub struct RouteItem {
    pub commingTime: i64,
    pub leaveTime: i64,
    pub pathname: String,
    pub url: String,
    pub route: String,
    pub searchObj: HashMap<String, String>,
}

impl RouteItem {
    pub fn get_cost_time(&self) -> i64 {
        self.leaveTime - self.commingTime
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
            created_at: Set(route.commingTime),
            leaved_at: Set(route.leaveTime),
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
            name: Set(name.clone()),
            created_at: Set(now),
            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    pub async fn get_route_name_count(&self, name: &String) -> Result<u64, sea_orm::DbErr> {
        routes::Entity::find()
            .filter(routes::Column::Route.eq(name))
            .count(&self.db)
            .await
    }

    pub async fn get_routes_count_by_route_and_date(
        &self,
        start_date: i64,
        end_date: i64,
    ) -> Result<u64, sea_orm::DbErr> {
        routes::Entity::find()
            .filter(routes::Column::CreatedAt.gte(start_date))
            .filter(routes::Column::CreatedAt.lte(end_date))
            .count(&self.db)
            .await
    }
}
