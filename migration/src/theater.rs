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
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum ShowtimeRoom {
    Table,
    Id,
    Time,
    Price,
    RoomId,
    ShowtimeId,
}

#[derive(DeriveIden)]
pub enum TakenSeat {
    Table,
    Id,
    ShowtimeId,
    ShowtimeRoomId,
    SeatIdentifier,
}
