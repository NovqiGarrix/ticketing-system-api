use crate::movie::Movie;
use crate::theater::{Room, Showtime, TakenSeat, Theater};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create movies table
        let _ = manager
            .create_table(
                Table::create()
                    .table(Movie::Table)
                    .if_not_exists()
                    .col(pk_uuid(Movie::Id).not_null())
                    .col(date(Movie::ReleaseDate).not_null())
                    .col(string(Movie::Title).not_null())
                    .col(string(Movie::Overview).not_null())
                    .col(float(Movie::Popularity).not_null())
                    .col(string(Movie::Genre).not_null())
                    .col(text(Movie::PosterUrl).not_null())
                    .to_owned(),
            )
            .await?;

        // Create theaters table
        let _ = manager
            .create_table(
                Table::create()
                    .table(Theater::Table)
                    .if_not_exists()
                    .col(pk_uuid(Theater::Id).not_null())
                    .col(string(Theater::Name).not_null())
                    .col(string(Theater::Location).not_null())
                    .to_owned(),
            )
            .await?;

        // Create rooms table
        let _ = manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(pk_uuid(Room::Id).not_null())
                    .col(string(Room::Name).not_null())
                    .col(integer(Room::Capacity).not_null())
                    .col(integer(Room::MaxRows).not_null())
                    .col(integer(Room::MaxColumns).not_null())
                    .col(uuid(Room::TheaterId).not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_rooms_theater_id")
                            .from_tbl(Room::Table)
                            .from_col(Room::TheaterId)
                            .to_tbl(Theater::Table)
                            .to_col(Theater::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create showtime table
        let _ = manager
            .create_table(
                Table::create()
                    .table(Showtime::Table)
                    .if_not_exists()
                    .col(pk_uuid(Showtime::Id).not_null())
                    .col(uuid(Showtime::MovieId).not_null())
                    .col(uuid(Showtime::RoomId).not_null())
                    .col(timestamp(Showtime::Time).not_null())
                    .col(money(Showtime::Price).not_null())
                    .col(
                        date_time(Showtime::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(Showtime::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_showtime_movie")
                            .from_tbl(Showtime::Table)
                            .from_col(Showtime::MovieId)
                            .to_tbl(Movie::Table)
                            .to_col(Movie::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_showtime_room")
                            .from_tbl(Showtime::Table)
                            .from_col(Showtime::RoomId)
                            .to_tbl(Room::Table)
                            .to_col(Room::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create taken_seats table
        let _ = manager
            .create_table(
                Table::create()
                    .table(TakenSeat::Table)
                    .if_not_exists()
                    .col(pk_auto(TakenSeat::Id).not_null())
                    .col(string_len(TakenSeat::SeatIdentifier, 4).not_null())
                    .col(uuid(TakenSeat::ShowtimeId).not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fs_taken_seats_showtime")
                            .from_tbl(TakenSeat::Table)
                            .from_col(TakenSeat::ShowtimeId)
                            .to_tbl(Showtime::Table)
                            .to_col(Showtime::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index for taken_seats table
        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("uq_taken_seat_showtime_id_seat_identifier")
                    .table(TakenSeat::Table)
                    .col(TakenSeat::ShowtimeId)
                    .col(TakenSeat::SeatIdentifier)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .drop_index(
                IndexDropStatement::new()
                    .if_exists()
                    .name("uq_taken_seat_showtime_id_seat_identifier")
                    .table(TakenSeat::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Movie::Table)
                    .table(TakenSeat::Table)
                    .table(Showtime::Table)
                    .table(Room::Table)
                    .table(Theater::Table)
                    .to_owned(),
            )
            .await
    }
}
