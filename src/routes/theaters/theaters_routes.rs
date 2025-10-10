use crate::app_state::AppState;
use actix_web::{get, web, HttpResponse};
use serde_json::json;

#[get("")]
pub async fn get_theaters_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let app_state = app_state.into_inner();
    let db = &app_state.database_connection;

    HttpResponse::Ok().json(json!({
        "status": "OK"
    }))
}
