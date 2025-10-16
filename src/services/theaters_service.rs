use std::str::FromStr;

use entity::theater;
use sea_orm::{DatabaseConnection, EntityTrait, raw_sql};
use uuid::Uuid;

use crate::{
    app_state::Result,
    models::{showtime_model::Showtime, theater_model::Theater},
};

use super::showtime_service::map_showtime;

pub async fn get_theaters(db: &DatabaseConnection) -> Result<Vec<Theater>> {
    let theaters = theater::Entity::find()
        .all(db)
        .await?
        .iter()
        .map(|t| Theater {
            id: t.id.to_string(),
            name: t.name.to_owned(),
            location: t.location.to_owned(),
        })
        .collect();

    Ok(theaters)
}

pub async fn get_theater_showtime(
    db: &DatabaseConnection,
    theater_id: String,
) -> Result<Vec<Showtime>> {
    let theater_id = Uuid::from_str(&theater_id)?;

    let showtime_query_results = theater::Entity::find()
        .from_raw_sql(raw_sql!(
            Postgres,
            r#"
                 SELECT sh.id,
                        created_at,
                        updated_at,
                        m.id           as m_id,
                        m.title        as m_title,
                        m.rating   as m_rating,
                        m.genre        as m_genre,
                        m.poster_url   as m_poster_url,
                        shr.id         as shr_id,
                        shr.time       as shr_time,
                        shr.price      as shr_price,
                        r.id           as shr_room_id,
                        r.name         as shr_room_name,
                        t.id           as t_id,
                        t.name         as t_name,
                        t.location     as t_location
                 FROM showtime sh
                          JOIN movie m ON m.id = sh.movie_id
                          JOIN showtime_room shr ON shr.showtime_id = sh.id
                          JOIN room r ON r.id = shr.room_id
                          JOIN theater t ON t.id = r.theater_id
                 WHERE t.id = {theater_id}
                 ORDER BY id, created_at DESC;
                 "#
        ))
        .into_json()
        .all(db)
        .await?;

    map_showtime(showtime_query_results).map_err(|e| e.into())
}
