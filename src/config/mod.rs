pub mod db;

use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialOrd, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Environment {
    Development,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

fn get_default_env() -> Environment {
    Environment::Development
}

fn get_default_port() -> u16 {
    8080
}

fn get_default_host() -> String {
    "127.0.0.1".to_string()
}

fn get_default_log_level() -> String {
    "info".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "get_default_env")]
    pub rust_env: Environment,
    #[serde(default = "get_default_log_level")]
    pub rust_log: String,
    #[serde(default = "get_default_port")]
    pub port: u16,
    #[serde(default = "get_default_host")]
    pub host: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().expect("Failed to load env variables from .env");
        envy::from_env()
            .expect("There are missing or invalid value for Config read from env variables")
    }

    pub fn rust_log_to_level_filter(&self) -> LevelFilter {
        match self.rust_log.as_str() {
            "info" => LevelFilter::Info,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            _ => LevelFilter::Debug,
        }
    }

    pub fn setup_log(&self) {
        env_logger::builder()
            .filter(None, self.rust_log_to_level_filter())
            .init();
    }
}
