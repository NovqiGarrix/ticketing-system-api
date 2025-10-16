use anyhow::anyhow;
use entity::movie;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use serde::Serialize;

use crate::{
    app_state::Result,
    models::{movie_model::Movie, requests::get_movies_request_model::GetMoviesQueryParams},
};

fn string_to_column(str: &str) -> core::result::Result<movie::Column, anyhow::Error> {
    match str {
        "id" => Ok(movie::Column::Id),
        "title" => Ok(movie::Column::Title),
        "overview" => Ok(movie::Column::Overview),
        "genre" => Ok(movie::Column::Genre),
        "rating" => Ok(movie::Column::Rating),
        _ => Err(anyhow!("{str} is not a valid movie column")),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub current_page: u64,
    pub total_page: u64,
}

pub async fn get_movies(
    db: &DatabaseConnection,
    options: GetMoviesQueryParams,
) -> Result<(Vec<Movie>, Info)> {
    let sort_options = options.sort_options;

    let sort_column_as_column = string_to_column(&sort_options.0)?;

    let paginator = movie::Entity::find()
        .order_by(sort_column_as_column, sort_options.1)
        .paginate(db, options.limit);

    let total_page = paginator.num_pages().await?;
    let movies = paginator
        .fetch_page(options.page)
        .await?
        .iter()
        .map(|m| Movie {
            id: m.id.to_string(),
            title: m.title.to_owned(),
            overview: m.overview.to_owned(),
            genre: m.genre.to_owned(),
            poster_url: m.poster_url.to_owned(),
            rating: m.rating,
        })
        .collect();

    Ok((
        movies,
        Info {
            current_page: options.page,
            // Somehow, if the current page is the last page,
            // no movies found; so we decrement the total page by 1
            total_page: total_page - 1,
        },
    ))
}
