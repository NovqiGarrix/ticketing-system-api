mod showtime_routes;

use crate::routes::showtime::showtime_routes::get_showtime_handler;
use actix_web::web::{ServiceConfig, scope};
use showtime_routes::get_taken_seats_handler;

pub fn showtime_routes(config: &mut ServiceConfig) {
    config.service(
        scope("/showtime")
            .service(get_showtime_handler)
            .service(get_taken_seats_handler),
    );
}
