use crate::app_error::AppError;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub database_connection: DatabaseConnection,
}

pub type Result<T> = core::result::Result<T, AppError>;
