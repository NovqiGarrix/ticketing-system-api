use crate::config::{Config, Environment};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn get_database_connection(config: &Config) -> DatabaseConnection {
    let mut options = ConnectOptions::new(config.database_url.to_owned());

    options
        .max_connections(15)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(Environment::Development.eq(&config.rust_env));

    Database::connect(options)
        .await
        .expect("Failed to connect to Database")
}
