use sea_orm::DatabaseConnection;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: redis::Client,
    pub tera: Tera,
}
