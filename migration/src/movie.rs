use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Movie {
    Table,
    Id,
    Title,
    Overview,
    Rating,
    Genre,
    PosterUrl,
}
