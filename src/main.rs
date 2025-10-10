mod app_state;
mod config;
mod routes;

use crate::app_state::AppState;
use crate::config::db::get_database_connection;
use crate::config::Config;
use actix_web::middleware::Logger;
use actix_web::{get, http::StatusCode, main, web, App, HttpResponse, HttpServer};
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
        config: config.clone(),
        database_connection,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(hello_world)
            .service(web::scope("/api").configure(routes::routes))
    })
    .bind((config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
