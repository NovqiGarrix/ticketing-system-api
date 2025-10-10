use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Theater {
    Table,
    Id,
    Name,
    Location,
}

#[derive(DeriveIden)]
pub enum Room {
    Table,
    Id,
    Name,
    Capacity,
    MaxRows,
    MaxColumns,
    TheaterId,
}

#[derive(DeriveIden)]
pub enum Showtime {
    Table,
    Id,
    MovieId,
    // TheaterId, we can add this, but since RoomId already related to the theater, we don't need this
    RoomId,
    Time,
    Price,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum TakenSeat {
    Table,
    Id,
    ShowtimeId,
    SeatIdentifier,
}
