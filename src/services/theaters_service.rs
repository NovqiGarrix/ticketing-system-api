use entity::theater;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{app_error::AppError, models::theater_model::Theater};

pub async fn get_theaters(db: &DatabaseConnection) -> Result<Vec<Theater>, AppError> {
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
