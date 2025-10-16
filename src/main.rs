mod app_error;
mod app_state;
mod config;
mod models;
mod routes;
mod services;

use crate::app_state::AppState;
use crate::config::Config;
use crate::config::db::get_database_connection;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{App, HttpResponse, HttpServer, get, http::StatusCode, main, web};
use serde_json::json;

#[get("/")]
async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "code": StatusCode::OK.as_u16(),
        "status": "OK"
    }))
}

#[main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    config.setup_log();

    let database_connection = get_database_connection(&config).await;

    let app_state = web::Data::new(AppState {
        database_connection,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(hello_world)
            .service(web::scope("/api").configure(routes::routes))
    })
    .bind((config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
