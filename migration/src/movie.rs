use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Movie {
    Table,
    Id,
    ReleaseDate,
    Title,
    Overview,
    Popularity,
    Genre,
    PosterUrl,
}
