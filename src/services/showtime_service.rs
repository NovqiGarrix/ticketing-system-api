use anyhow::{Context, anyhow};
use chrono::NaiveDateTime;
use entity::{showtime, taken_seat};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, raw_sql};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

use crate::{
    app_state::Result,
    models::showtime_model::{Movie, Showtime, ShowtimeRoom, Theater},
};

pub fn map_showtime(query_results: Vec<serde_json::Value>) -> Result<Vec<Showtime>> {
    if query_results.is_empty() {
        return Ok(vec![]);
    }

    let mut results: Vec<Showtime> = vec![];
    let mut grouped_results: HashMap<String, Vec<Value>> = HashMap::new();

    // Group all rows by the showtime ID first. This is simpler than comparing with the next item.
    for row in query_results {
        let id = row
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("'id' field is missing or not a string in query result"))?
            .to_string();
        grouped_results.entry(id).or_default().push(row);
    }

    for (id, rows) in grouped_results {
        let first_row = &rows[0]; // All rows in this group share the main showtime info.

        let mut theaters = HashMap::new();
        let mut showtime_rooms = HashMap::new();

        for row in &rows {
            // Use and_then to safely chain operations that return Option.
            let t_id = row.get("t_id").and_then(Value::as_str).unwrap_or_default();
            if !theaters.contains_key(t_id) {
                theaters.insert(
                    t_id.to_string(),
                    Theater {
                        id: t_id.to_string(),
                        name: row
                            .get("t_name")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_string(),
                        location: row
                            .get("t_location")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_string(),
                    },
                );
            }

            if let Some(shr_id) = row.get("shr_id").and_then(Value::as_u64) {
                if !showtime_rooms.contains_key(&shr_id) {
                    let shr_time_str = row
                        .get("shr_time")
                        .and_then(Value::as_str)
                        .ok_or_else(|| anyhow!("'shr_time' is missing or is not parseable"))?;

                    let shr = ShowtimeRoom {
                        id: shr_id,
                        price: row
                            .get("shr_price")
                            .and_then(Value::as_i64)
                            .ok_or_else(|| anyhow!("'shr_price' is missing or is not parseable"))?
                            as u32,
                        time: NaiveDateTime::parse_from_str(shr_time_str, "%Y-%m-%dT%H:%M:%S")
                            .context(format!("Failed to parse shr_time: {}", shr_time_str))?,
                        room_id: row
                            .get("shr_room_id")
                            .and_then(Value::as_str)
                            .ok_or_else(|| anyhow!("'shr_room_id' is missing or is not parseable"))?
                            .to_string(),
                        room_name: row
                            .get("shr_room_name")
                            .and_then(Value::as_str)
                            .ok_or_else(|| {
                                anyhow!("'shr_room_name' is missing or is not parseable")
                            })?
                            .to_string(),
                    };
                    showtime_rooms.insert(shr_id, shr);
                }
            }
        }

        let created_at_str = first_row
            .get("created_at")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("'created_at' is missing"))?;
        let updated_at_str = first_row
            .get("updated_at")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("'updated_at' is missing"))?;

        let sh = Showtime {
            id,
            created_at: NaiveDateTime::parse_from_str(created_at_str, "%Y-%m-%dT%H:%M:%S.%f")
                .context(format!("Failed to parse created_at: {}", created_at_str))?,
            updated_at: NaiveDateTime::parse_from_str(updated_at_str, "%Y-%m-%dT%H:%M:%S.%f")
                .context(format!("Failed to parse updated_at: {}", updated_at_str))?,
            movie: Movie {
                id: first_row
                    .get("m_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                title: first_row
                    .get("m_title")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                genre: first_row
                    .get("m_genre")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                rating: first_row
                    .get("m_rating")
                    .and_then(Value::as_f64)
                    .unwrap_or_default(),
                poster_url: first_row
                    .get("m_poster_url")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
            },
            theaters: theaters.into_values().collect(),
            showtime_rooms: showtime_rooms.into_values().collect(),
        };

        results.push(sh);
    }

    Ok(results)
}

pub async fn get_showtime(db: &DatabaseConnection) -> Result<Vec<Showtime>> {
    let showtime_query_results = showtime::Entity::find()
        .from_raw_sql(raw_sql!(
            Postgres,
            r#"
        SELECT sh.id,
       created_at,
       updated_at,
       m.id           as m_id,
       m.title        as m_title,
       m.rating       as m_rating,
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
ORDER BY created_at DESC;
        "#
        ))
        .into_json()
        .all(db)
        .await?;

    map_showtime(showtime_query_results).map_err(|e| e.into())
}

pub async fn get_taken_seats(
    db: &DatabaseConnection,
    showtime_id: String,
    showtime_room_id: i32,
) -> Result<Vec<String>> {
    let showtime_id = Uuid::from_str(&showtime_id)?;

    let seat_taken = taken_seat::Entity::find()
        .filter(
            Condition::all()
                .add(taken_seat::Column::ShowtimeId.eq(showtime_id))
                .add(taken_seat::Column::ShowtimeRoomId.eq(showtime_room_id)),
        )
        .all(db)
        .await?;

    let seat_taken = seat_taken
        .iter()
        .map(|st| st.seat_identifier.to_owned())
        .collect::<Vec<_>>();

    Ok(seat_taken)
}
