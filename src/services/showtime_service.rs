use chrono::{NaiveDate, NaiveDateTime};
use entity::showtime;
use sea_orm::{DatabaseConnection, EntityTrait, raw_sql};
use std::{collections::HashMap, io::Error};

use crate::models::showtime_model::{Movie, Showtime, ShowtimeRoom, Theater};

pub async fn get_showtime(db: &DatabaseConnection) -> Result<Vec<Showtime>, Error> {
    let showtime_query_results = showtime::Entity::find()
        .from_raw_sql(raw_sql!(
            Postgres,
            r#"
        SELECT sh.id,
       created_at,
       updated_at,
       m.id           as m_id,
       m.release_date as m_release_date,
       m.title        as m_title,
       m.popularity   as m_popularity,
       m.genre        as m_genre,
       m.poster_url   as m_poster_url,
       shr.id         as shr_id,
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
        .await
        .unwrap();

    let mut shr_hash_map: HashMap<String, Vec<ShowtimeRoom>> = HashMap::new();
    let mut theater_hash_map: HashMap<String, Vec<Theater>> = HashMap::new();

    let mut results: Vec<Showtime> = vec![];

    for i in 0..showtime_query_results.len() {
        let showtime = &showtime_query_results[i];

        let id: String = showtime["id"].as_str().unwrap().to_string();

        let default_showtime_rooms = vec![];
        let default_theaters = vec![];

        let mut prev_showtime_rooms = shr_hash_map
            .get(id.as_str())
            .unwrap_or(&default_showtime_rooms)
            .to_vec();
        let showtime_room = ShowtimeRoom {
            id: showtime["shr_id"].as_u64().unwrap() as u32,
        };
        prev_showtime_rooms.push(showtime_room);
        shr_hash_map.insert(id.to_owned(), prev_showtime_rooms.to_vec());

        let mut prev_theaters = theater_hash_map
            .get(id.as_str())
            .unwrap_or(&default_theaters)
            .to_vec();
        let theater = Theater {
            id: showtime["t_id"].as_str().unwrap().to_string(),
            name: showtime["t_name"].as_str().unwrap().to_string(),
            location: showtime["t_location"].as_str().unwrap().to_string(),
        };
        prev_theaters.push(theater);
        theater_hash_map.insert(id.to_owned(), prev_theaters.to_vec());

        let next_id = if i + 1 >= showtime_query_results.len() {
            "".to_owned()
        } else {
            showtime_query_results[i + 1]["id"]
                .as_str()
                .unwrap()
                .to_owned()
        };

        if id != next_id {
            // All showrooms and theaters
            // has been put into the hash maps

            // Collect showtime rooms
            let shrs = shr_hash_map.get(&id).unwrap().to_vec();
            // Collect theaters
            let theaters = theater_hash_map.get(&id).unwrap().to_vec();

            let sh = Showtime {
                id: id.to_owned(),
                created_at: NaiveDateTime::parse_from_str(
                    &showtime["created_at"].as_str().unwrap(),
                    "%Y-%m-%dT%H:%M:%S.%f",
                )
                .unwrap(),
                updated_at: NaiveDateTime::parse_from_str(
                    &showtime["updated_at"].as_str().unwrap(),
                    "%Y-%m-%dT%H:%M:%S.%f",
                )
                .unwrap(),
                movie: Movie {
                    id: showtime["m_id"].as_str().unwrap().to_string(),
                    title: showtime["m_title"].as_str().unwrap().to_string(),
                    genre: showtime["m_genre"].as_str().unwrap().to_string(),
                    popularity: showtime["m_popularity"].as_f64().unwrap(),
                    poster_url: showtime["m_poster_url"].as_str().unwrap().to_string(),
                    release_date: NaiveDate::parse_from_str(
                        showtime["m_release_date"].as_str().unwrap(),
                        "%Y-%m-%d",
                    )
                    .unwrap(),
                },
                showtime_rooms: shrs,
                theaters: theaters,
            };

            results.push(sh);
        }
    }

    Ok(results)
}
