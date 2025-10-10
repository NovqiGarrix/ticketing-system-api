use crate::config::Config;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub config: Config,
    pub database_connection: DatabaseConnection,
}
