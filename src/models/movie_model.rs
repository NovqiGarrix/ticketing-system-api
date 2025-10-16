use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub overview: String,
    pub rating: f32,
    pub genre: String,
    pub poster_url: String,
}
