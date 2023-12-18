use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: redis::Client,
}
