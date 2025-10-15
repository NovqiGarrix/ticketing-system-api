use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ShowtimeRoom {
    pub id: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub id: String,
    pub release_date: NaiveDate,
    pub title: String,
    pub popularity: f64,
    pub genre: String,
    pub poster_url: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Theater {
    pub id: String,
    pub name: String,
    pub location: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Showtime {
    pub id: String,

    pub created_at: NaiveDateTime,

    pub updated_at: NaiveDateTime,

    pub movie: Movie,

    pub showtime_rooms: Vec<ShowtimeRoom>,

    pub theaters: Vec<Theater>,
}
