use actix_web::{
    HttpResponse, get,
    http::StatusCode,
    web::{Data, Path, Query},
};
use serde_json::json;

use crate::{
    app_state::{AppState, Result},
    models::requests::get_movies_request_model::GetMoviesQueryParams,
    services::movies_service::{get_movie, get_movies},
};

#[get("")]
pub async fn get_movies_handler(
    app_state: Data<AppState>,
    query_params: Query<GetMoviesQueryParams>,
) -> Result<HttpResponse> {
    let (movies, info) =
        get_movies(&app_state.database_connection, query_params.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": StatusCode::OK.as_u16(),
        "data": movies,
        "info": info
    })))
}

#[get("/{movie_id}")]
pub async fn get_movie_handler(
    app_state: Data<AppState>,
    movie_id: Path<String>,
) -> Result<HttpResponse> {
    let movie = get_movie(&app_state.database_connection, movie_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": StatusCode::OK.as_u16(),
        "data": movie
    })))
}
