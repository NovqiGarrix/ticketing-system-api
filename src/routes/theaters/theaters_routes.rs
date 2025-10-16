use crate::{
    app_state::{AppState, Result},
    services::theaters_service::{get_theater_showtime, get_theaters},
};
use actix_web::{
    HttpResponse, get,
    http::StatusCode,
    web::{Data, Path},
};
use serde_json::json;

#[get("")]
pub async fn get_theaters_handler(app_state: Data<AppState>) -> Result<HttpResponse> {
    let theaters = get_theaters(&app_state.database_connection).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "OK",
        "data": theaters
    })))
}

#[get("/{theater_id}/showtime")]
pub async fn get_theater_showtime_handler(
    app_state: Data<AppState>,
    theater_id: Path<String>,
) -> Result<HttpResponse> {
    let showtime =
        get_theater_showtime(&app_state.database_connection, theater_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": StatusCode::OK.as_u16(),
        "data": showtime
    })))
}
