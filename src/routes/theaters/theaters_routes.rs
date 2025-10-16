use crate::{
    app_state::{AppState, Result},
    services::theaters_service::get_theaters,
};
use actix_web::{HttpResponse, get, web};
use serde_json::json;

#[get("")]
pub async fn get_theaters_handler(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let theaters = get_theaters(&app_state.database_connection).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "OK",
        "data": theaters
    })))
}
