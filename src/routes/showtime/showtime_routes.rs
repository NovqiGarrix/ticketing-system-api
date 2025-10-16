use crate::app_state::{AppState, Result};
use crate::services::showtime_service::get_showtime;
use actix_web::{HttpResponse, get, web::Data};
use serde_json::json;

#[get("")]
pub async fn get_showtime_handler(state: Data<AppState>) -> Result<HttpResponse> {
    let showtime = get_showtime(&state.database_connection).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "OK",
        "data": showtime
    })))
}
