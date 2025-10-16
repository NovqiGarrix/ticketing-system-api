use crate::app_state::{AppState, Result};
use crate::services::showtime_service::{get_showtime, get_taken_seats};
use actix_web::http::StatusCode;
use actix_web::web::Path;
use actix_web::{HttpResponse, get, web::Data};
use serde::Deserialize;
use serde_json::json;

#[get("")]
pub async fn get_showtime_handler(state: Data<AppState>) -> Result<HttpResponse> {
    let showtime = get_showtime(&state.database_connection).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "OK",
        "data": showtime
    })))
}

#[derive(Deserialize)]
struct GetTakenSeatsPath {
    showtime_id: String,
    showtime_room_id: i32,
}

#[get("/{showtime_id}/showtime-rooms/{showtime_room_id}/taken-seats")]
pub async fn get_taken_seats_handler(
    state: Data<AppState>,
    path: Path<GetTakenSeatsPath>,
) -> Result<HttpResponse> {
    let path = path.into_inner();

    let taken_seats = get_taken_seats(
        &state.database_connection,
        path.showtime_id,
        path.showtime_room_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": StatusCode::OK.as_u16(),
        "data": taken_seats
    })))
}
